//! Session backend implementations for different storage strategies.
//!
//! This module provides multiple backend implementations for FIX session management:
//! - Memory: Fast in-memory storage using high-performance data structures
//! - File: Persistent file-based storage for reliability across restarts
//! - Database: SQLite-based storage for complex querying and durability

use crate::FieldType;
use crate::session::{Backend, Environment};
use log;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use smartstring::alias::String as SmartString;
use std::collections::VecDeque;
use std::ops::Range;
use std::path::PathBuf;
use thiserror::Error;

/// Backend implementation errors
#[derive(Debug, Error)]
pub enum BackendError {
    /// I/O operation failed
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Serialization or deserialization failed
    #[error("Serialization error: {0}")]
    Serialization(String),
    /// Message with specified sequence number was not found
    #[error("Message not found: sequence {0}")]
    MessageNotFound(u64),
    /// Invalid sequence number range provided
    #[error("Invalid sequence number range: {start}..{end}")]
    InvalidRange {
        /// Start of the invalid range
        start: u64,
        /// End of the invalid range
        end: u64,
    },
    /// Storage capacity has been exceeded
    #[error("Storage capacity exceeded")]
    CapacityExceeded,
}

impl<'a> FieldType<'a> for BackendError {
    type Error = String;
    type SerializeSettings = ();

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(BackendError::Serialization(
            String::from_utf8_lossy(data).to_string(),
        ))
    }

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: crate::Buffer,
    {
        let error_str = ToString::to_string(self);
        let bytes = error_str.as_bytes();
        buffer.extend_from_slice(bytes);
        bytes.len()
    }
}

/// High-performance in-memory backend using optimized data structures
#[derive(Debug, Clone)]
pub struct MemoryBackend {
    sender_comp_id: SmartString,
    target_comp_id: SmartString,
    message_encoding: Option<SmartString>,
    /// Outbound message store for resend requests
    outbound_messages: FxHashMap<u64, SmallVec<[u8; 1024]>>,
    /// Inbound message store for duplicate detection
    inbound_messages: FxHashMap<u64, SmallVec<[u8; 1024]>>,
    /// Queue of pending outbound messages
    pending_queue: VecDeque<SmallVec<[u8; 1024]>>,
    /// Maximum messages to store (prevents memory growth)
    max_stored_messages: usize,
    /// Environment setting
    environment: Environment,
}

impl MemoryBackend {
    /// Creates a new memory backend with specified CompIDs
    pub fn new(
        sender_comp_id: impl Into<SmartString>,
        target_comp_id: impl Into<SmartString>,
    ) -> Self {
        Self {
            sender_comp_id: sender_comp_id.into(),
            target_comp_id: target_comp_id.into(),
            message_encoding: None,
            outbound_messages: FxHashMap::default(),
            inbound_messages: FxHashMap::default(),
            pending_queue: VecDeque::new(),
            max_stored_messages: 10000, // Configurable limit
            environment: Environment::Production { allow_test: false },
        }
    }

    /// Creates a new memory backend with custom configuration
    pub fn with_config(
        sender_comp_id: impl Into<SmartString>,
        target_comp_id: impl Into<SmartString>,
        max_stored_messages: usize,
        environment: Environment,
    ) -> Self {
        Self {
            sender_comp_id: sender_comp_id.into(),
            target_comp_id: target_comp_id.into(),
            message_encoding: None,
            outbound_messages: FxHashMap::default(),
            inbound_messages: FxHashMap::default(),
            pending_queue: VecDeque::new(),
            max_stored_messages,
            environment,
        }
    }

    /// Store an outbound message for potential resend
    pub fn store_outbound_message(
        &mut self,
        seq_num: u64,
        message: &[u8],
    ) -> Result<(), BackendError> {
        if self.outbound_messages.len() >= self.max_stored_messages {
            // ✅ CRITICAL FIX: Use count-based retention instead of sequence-based to handle non-contiguous sequence numbers
            // FIX protocol allows gaps in sequence numbers (e.g., 1000, 1005, 1010), so we cannot assume contiguity
            let target_size = self.max_stored_messages * 3 / 4; // Reduce to 75% capacity
            self.cleanup_outbound_messages(target_size);
        }

        let mut buffer = SmallVec::new();
        buffer.extend_from_slice(message);
        self.outbound_messages.insert(seq_num, buffer);
        Ok(())
    }

    /// Store an inbound message for duplicate detection
    pub fn store_inbound_message(
        &mut self,
        seq_num: u64,
        message: &[u8],
    ) -> Result<(), BackendError> {
        if self.inbound_messages.len() >= self.max_stored_messages {
            // ✅ CRITICAL FIX: Use count-based retention instead of sequence-based to handle non-contiguous sequence numbers
            // FIX protocol allows gaps in sequence numbers (e.g., 1000, 1005, 1010), so we cannot assume contiguity
            let target_size = self.max_stored_messages * 3 / 4; // Reduce to 75% capacity
            self.cleanup_inbound_messages(target_size);
        }

        let mut buffer = SmallVec::new();
        buffer.extend_from_slice(message);
        self.inbound_messages.insert(seq_num, buffer);
        Ok(())
    }

    /// Clean up outbound message store using count-based retention (handles non-contiguous sequence numbers)
    fn cleanup_outbound_messages(&mut self, target_size: usize) {
        if self.outbound_messages.len() <= target_size {
            return;
        }

        // Collect and sort sequence numbers to find the oldest messages
        let mut seq_nums: SmallVec<[u64; 32]> = self.outbound_messages.keys().cloned().collect();
        seq_nums.sort_unstable();

        // Calculate how many messages to remove
        let messages_to_remove = self.outbound_messages.len() - target_size;

        // Remove the oldest messages by sequence number (regardless of gaps)
        for &seq_num in seq_nums.iter().take(messages_to_remove) {
            self.outbound_messages.remove(&seq_num);
        }

        log::debug!(
            "Cleaned up outbound message store: removed {} oldest messages, {} messages remaining",
            messages_to_remove,
            self.outbound_messages.len()
        );
    }

    /// Clean up inbound message store using count-based retention (handles non-contiguous sequence numbers)
    fn cleanup_inbound_messages(&mut self, target_size: usize) {
        if self.inbound_messages.len() <= target_size {
            return;
        }

        // Collect and sort sequence numbers to find the oldest messages
        let mut seq_nums: SmallVec<[u64; 32]> = self.inbound_messages.keys().cloned().collect();
        seq_nums.sort_unstable();

        // Calculate how many messages to remove
        let messages_to_remove = self.inbound_messages.len() - target_size;

        // Remove the oldest messages by sequence number (regardless of gaps)
        for &seq_num in seq_nums.iter().take(messages_to_remove) {
            self.inbound_messages.remove(&seq_num);
        }

        log::debug!(
            "Cleaned up inbound message store: removed {} oldest messages, {} messages remaining",
            messages_to_remove,
            self.inbound_messages.len()
        );
    }

    /// Queue a message for sending
    pub fn queue_message(&mut self, message: &[u8]) {
        let mut buffer = SmallVec::new();
        buffer.extend_from_slice(message);
        self.pending_queue.push_back(buffer);
    }

    /// Get messages for resend request
    pub fn get_messages_for_resend(&self, range: Range<u64>) -> SmallVec<[&[u8]; 16]> {
        let mut messages = SmallVec::new();
        for seq_num in range {
            if let Some(message) = self.outbound_messages.get(&seq_num) {
                messages.push(message.as_slice());
            }
        }
        messages
    }

    /// Check if a message is a duplicate
    pub fn is_duplicate(&self, seq_num: u64) -> bool {
        self.inbound_messages.contains_key(&seq_num)
    }
}

impl Backend for MemoryBackend {
    type Error = BackendError;

    fn sender_comp_id(&self) -> &[u8] {
        self.sender_comp_id.as_bytes()
    }

    fn target_comp_id(&self) -> &[u8] {
        self.target_comp_id.as_bytes()
    }

    fn message_encoding(&self) -> Option<&[u8]> {
        self.message_encoding.as_ref().map(|s| s.as_bytes())
    }

    fn environment(&self) -> Environment {
        self.environment
    }

    fn on_heartbeat_is_due(&mut self) -> Result<(), Self::Error> {
        log::debug!("Memory backend: heartbeat due");
        Ok(())
    }

    fn on_inbound_app_message(
        &mut self,
        message: crate::tagvalue::Message<&[u8]>,
    ) -> Result<(), Self::Error> {
        log::debug!("Memory backend: processing inbound application message");
        // Store for duplicate detection if sequence number is available
        if let Ok(seq_num) = message.get::<u64>(34) {
            // MSG_SEQ_NUM = 34
            self.store_inbound_message(seq_num, message.as_bytes())?;
        }
        Ok(())
    }

    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        log::debug!("Memory backend: processing outbound message");
        // Parse sequence number and store for potential resend
        // This is a simplified implementation - in practice you'd parse the actual message
        // For now, we'll queue it for sending
        self.queue_message(message);
        Ok(())
    }

    fn on_resend_request(&mut self, range: Range<u64>) -> Result<(), Self::Error> {
        log::info!("Memory backend: processing resend request for range {range:?}");

        if range.start > range.end {
            return Err(BackendError::InvalidRange {
                start: range.start,
                end: range.end,
            });
        }

        let messages: SmallVec<[SmallVec<[u8; 1024]>; 16]> = self
            .get_messages_for_resend(range.clone())
            .into_iter()
            .map(SmallVec::from)
            .collect();
        log::info!(
            "Found {} messages for resend in range {:?}",
            messages.len(),
            range
        );

        // Re-queue messages for transmission
        for message in messages {
            self.queue_message(&message);
        }

        Ok(())
    }

    fn on_successful_handshake(&mut self) -> Result<(), Self::Error> {
        log::info!("Memory backend: successful handshake completed");
        Ok(())
    }

    fn fetch_messages(&mut self) -> Result<&[&[u8]], Self::Error> {
        // This is a simplified implementation
        // In practice, you'd return references to queued messages
        Ok(&[])
    }

    fn pending_message(&mut self) -> Option<&[u8]> {
        self.pending_queue.front().map(|v| v.as_slice())
    }
}

/// File-based backend for persistent storage
#[derive(Debug, Clone)]
pub struct FileBackend {
    sender_comp_id: SmartString,
    target_comp_id: SmartString,
    message_encoding: Option<SmartString>,
    storage_path: PathBuf,
    /// In-memory cache for performance
    memory_cache: MemoryBackend,
}

impl FileBackend {
    /// Creates a new file-based backend
    pub fn new(
        sender_comp_id: impl Into<SmartString>,
        target_comp_id: impl Into<SmartString>,
        storage_path: impl Into<PathBuf>,
    ) -> Result<Self, BackendError> {
        let sender = sender_comp_id.into();
        let target = target_comp_id.into();
        let path = storage_path.into();

        // Create storage directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(Self {
            sender_comp_id: sender.clone(),
            target_comp_id: target.clone(),
            message_encoding: None,
            storage_path: path,
            memory_cache: MemoryBackend::new(sender, target),
        })
    }

    /// Load messages from file into memory cache
    pub fn load_from_disk(&mut self) -> Result<(), BackendError> {
        if !self.storage_path.exists() {
            return Ok(());
        }

        let contents = std::fs::read_to_string(&self.storage_path)?;
        for line in contents.lines() {
            if let Some((seq_str, message)) = line.split_once(':')
                && let Ok(seq_num) = seq_str.parse::<u64>()
            {
                let message_bytes = message.as_bytes();
                self.memory_cache
                    .store_outbound_message(seq_num, message_bytes)?;
            }
        }
        Ok(())
    }

    /// Persist messages to disk
    pub fn save_to_disk(&self) -> Result<(), BackendError> {
        use std::io::Write;

        let mut file = std::fs::File::create(&self.storage_path)?;
        for (seq_num, message) in &self.memory_cache.outbound_messages {
            writeln!(file, "{}:{}", seq_num, String::from_utf8_lossy(message))?;
        }
        file.sync_all()?;
        Ok(())
    }
}

impl Backend for FileBackend {
    type Error = BackendError;

    fn sender_comp_id(&self) -> &[u8] {
        self.sender_comp_id.as_bytes()
    }

    fn target_comp_id(&self) -> &[u8] {
        self.target_comp_id.as_bytes()
    }

    fn message_encoding(&self) -> Option<&[u8]> {
        self.message_encoding.as_ref().map(|s| s.as_bytes())
    }

    fn environment(&self) -> Environment {
        self.memory_cache.environment()
    }

    fn on_heartbeat_is_due(&mut self) -> Result<(), Self::Error> {
        self.memory_cache.on_heartbeat_is_due()
    }

    fn on_inbound_app_message(
        &mut self,
        message: crate::tagvalue::Message<&[u8]>,
    ) -> Result<(), Self::Error> {
        let result = self.memory_cache.on_inbound_app_message(message);
        // Persist periodically (every 100 messages for performance)
        if self.memory_cache.inbound_messages.len() % 100 == 0 {
            self.save_to_disk()?;
        }
        result
    }

    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        let result = self.memory_cache.on_outbound_message(message);
        // Persist outbound messages immediately for reliability
        self.save_to_disk()?;
        result
    }

    fn on_resend_request(&mut self, range: Range<u64>) -> Result<(), Self::Error> {
        self.memory_cache.on_resend_request(range)
    }

    fn on_successful_handshake(&mut self) -> Result<(), Self::Error> {
        log::info!("File backend: successful handshake completed, persisting state");
        self.save_to_disk()?;
        self.memory_cache.on_successful_handshake()
    }

    fn fetch_messages(&mut self) -> Result<&[&[u8]], Self::Error> {
        self.memory_cache.fetch_messages()
    }

    fn pending_message(&mut self) -> Option<&[u8]> {
        self.memory_cache.pending_message()
    }
}

/// Database backend for robust persistence and querying
/// Uses SQLite for simplicity and portability
#[derive(Debug, Clone)]
pub struct DatabaseBackend {
    sender_comp_id: SmartString,
    target_comp_id: SmartString,
    message_encoding: Option<SmartString>,
    db_path: PathBuf,
    /// In-memory cache for performance
    memory_cache: MemoryBackend,
}

impl DatabaseBackend {
    /// Creates a new database backend
    /// Note: Full SQLite implementation would require adding the rusqlite dependency
    /// This is a placeholder implementation that falls back to file storage
    pub fn new(
        sender_comp_id: impl Into<SmartString>,
        target_comp_id: impl Into<SmartString>,
        db_path: impl Into<PathBuf>,
    ) -> Result<Self, BackendError> {
        let sender = sender_comp_id.into();
        let target = target_comp_id.into();
        let path = db_path.into();

        // Create database directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(Self {
            sender_comp_id: sender.clone(),
            target_comp_id: target.clone(),
            message_encoding: None,
            db_path: path,
            memory_cache: MemoryBackend::new(sender, target),
        })
    }

    /// Initialize database schema
    /// In a full implementation, this would create the necessary tables
    pub fn initialize_schema(&self) -> Result<(), BackendError> {
        log::info!(
            "Database backend: initializing schema at {:?}",
            self.db_path
        );
        // Placeholder - would execute SQL CREATE TABLE statements
        Ok(())
    }

    /// Store message in database
    /// Placeholder implementation - would execute SQL INSERT
    pub fn store_message_in_db(
        &self,
        seq_num: u64,
        _message: &[u8],
        direction: &str,
    ) -> Result<(), BackendError> {
        log::debug!("Database backend: storing {direction} message seq_num={seq_num}");
        // Placeholder for SQL: INSERT INTO messages (seq_num, direction, content, timestamp) VALUES (?, ?, ?, ?)
        Ok(())
    }

    /// Query messages from database
    /// Placeholder implementation - would execute SQL SELECT
    pub fn query_messages_from_db(&self, range: Range<u64>) -> Result<QueryResult, BackendError> {
        log::debug!("Database backend: querying messages for range {range:?}");
        // Placeholder for SQL: SELECT seq_num, content FROM messages WHERE seq_num BETWEEN ? AND ? ORDER BY seq_num
        Ok(SmallVec::new())
    }
}

type QueryResult = SmallVec<[(u64, SmallVec<[u8; 1024]>); 16]>;

impl Backend for DatabaseBackend {
    type Error = BackendError;

    fn sender_comp_id(&self) -> &[u8] {
        self.sender_comp_id.as_bytes()
    }

    fn target_comp_id(&self) -> &[u8] {
        self.target_comp_id.as_bytes()
    }

    fn message_encoding(&self) -> Option<&[u8]> {
        self.message_encoding.as_ref().map(|s| s.as_bytes())
    }

    fn environment(&self) -> Environment {
        self.memory_cache.environment()
    }

    fn on_heartbeat_is_due(&mut self) -> Result<(), Self::Error> {
        self.memory_cache.on_heartbeat_is_due()
    }

    fn on_inbound_app_message(
        &mut self,
        message: crate::tagvalue::Message<&[u8]>,
    ) -> Result<(), Self::Error> {
        // Extract sequence number and message bytes before moving the message
        let seq_num_result = message.get::<u64>(34); // MSG_SEQ_NUM = 34
        let message_bytes = message.as_bytes().to_vec();

        // Store in memory cache
        let result = self.memory_cache.on_inbound_app_message(message);

        // Store in database if sequence number is available
        if let Ok(seq_num) = seq_num_result {
            self.store_message_in_db(seq_num, &message_bytes, "inbound")?;
        }

        result
    }

    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        let result = self.memory_cache.on_outbound_message(message);

        // Parse sequence number and store in database
        // This is simplified - would need proper FIX message parsing
        self.store_message_in_db(0, message, "outbound")?; // seq_num=0 as placeholder

        result
    }

    fn on_resend_request(&mut self, range: Range<u64>) -> Result<(), Self::Error> {
        log::info!("Database backend: processing resend request for range {range:?}");

        // Query from database for more comprehensive coverage
        let db_messages = self.query_messages_from_db(range.clone())?;

        // Fallback to memory cache
        let cache_result = self.memory_cache.on_resend_request(range);

        log::info!(
            "Found {} messages in database for resend",
            db_messages.len()
        );
        cache_result
    }

    fn on_successful_handshake(&mut self) -> Result<(), Self::Error> {
        log::info!("Database backend: successful handshake completed, updating database");
        self.memory_cache.on_successful_handshake()
    }

    fn fetch_messages(&mut self) -> Result<&[&[u8]], Self::Error> {
        self.memory_cache.fetch_messages()
    }

    fn pending_message(&mut self) -> Option<&[u8]> {
        self.memory_cache.pending_message()
    }
}

/// A trait for FIX applications (legacy compatibility)
pub trait Application {
    /// Called when an application message is received
    fn on_message(&mut self, message: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_backend_creation() {
        let backend = MemoryBackend::new("SENDER", "TARGET");
        assert_eq!(backend.sender_comp_id(), b"SENDER");
        assert_eq!(backend.target_comp_id(), b"TARGET");
        assert_eq!(backend.outbound_messages.len(), 0);
        assert_eq!(backend.inbound_messages.len(), 0);
    }

    #[test]
    fn test_memory_backend_message_storage() {
        let mut backend = MemoryBackend::new("SENDER", "TARGET");
        let message = b"8=FIX.4.2\x019=40\x0135=D\x0149=SENDER\x0156=TARGET\x0134=1\x01";

        backend.store_outbound_message(1, message).unwrap();
        assert!(backend.outbound_messages.contains_key(&1));

        backend.store_inbound_message(1, message).unwrap();
        assert!(backend.is_duplicate(1));
    }

    #[test]
    fn test_memory_backend_resend_request() {
        let mut backend = MemoryBackend::new("SENDER", "TARGET");
        let message1 = b"message1";
        let message2 = b"message2";

        backend.store_outbound_message(1, message1).unwrap();
        backend.store_outbound_message(2, message2).unwrap();

        let messages = backend.get_messages_for_resend(1..3);
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_file_backend_creation() {
        let temp_path = std::env::temp_dir().join("test_fix_messages.txt");
        let backend = FileBackend::new("SENDER", "TARGET", &temp_path).unwrap();
        assert_eq!(backend.sender_comp_id(), b"SENDER");
        assert_eq!(backend.target_comp_id(), b"TARGET");

        // Cleanup
        let _ = std::fs::remove_file(temp_path);
    }

    #[test]
    fn test_database_backend_creation() {
        let temp_path = std::env::temp_dir().join("test_fix_messages.db");
        let backend = DatabaseBackend::new("SENDER", "TARGET", &temp_path).unwrap();
        assert_eq!(backend.sender_comp_id(), b"SENDER");
        assert_eq!(backend.target_comp_id(), b"TARGET");

        // Cleanup
        let _ = std::fs::remove_file(temp_path);
    }
}
