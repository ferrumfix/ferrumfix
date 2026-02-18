use super::SeqNumbers;
use async_trait::async_trait;
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;
#[cfg(feature = "session-store-sqlite")]
use std::{io, path::Path, time::Duration};

/// Stable session identifier used by storage implementations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionKey {
    /// FIX begin string (e.g. `FIX.4.4`).
    pub begin_string: Vec<u8>,
    /// Local sender ID.
    pub sender_comp_id: Vec<u8>,
    /// Remote target ID.
    pub target_comp_id: Vec<u8>,
}

/// Persisted outbound application message metadata and bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredAppMessage {
    /// Outbound `MsgSeqNum <34>`.
    pub seq_num: u64,
    /// Raw FIX bytes sent on the wire.
    pub raw_message: Vec<u8>,
}

/// Durable storage contract for FIX session replay and sequence tracking.
#[async_trait]
pub trait SessionStore {
    /// Storage error type.
    type Error;

    /// Loads persisted sequence numbers for a session key.
    async fn load_seq_numbers(
        &mut self,
        key: &SessionKey,
    ) -> Result<Option<SeqNumbers>, Self::Error>;

    /// Persists current sequence numbers for a session key.
    async fn save_seq_numbers(
        &mut self,
        key: &SessionKey,
        seq: SeqNumbers,
    ) -> Result<(), Self::Error>;

    /// Persists one outbound application message.
    async fn save_outbound_app(
        &mut self,
        key: &SessionKey,
        msg: StoredAppMessage,
    ) -> Result<(), Self::Error>;

    /// Loads outbound application messages in the inclusive sequence range.
    async fn load_outbound_app_range(
        &mut self,
        key: &SessionKey,
        start_inclusive: u64,
        end_inclusive: u64,
    ) -> Result<Vec<StoredAppMessage>, Self::Error>;

    /// Clears or resets session state after sequence reset semantics.
    async fn reset_session(&mut self, key: &SessionKey, seq: SeqNumbers)
        -> Result<(), Self::Error>;
}

/// In-memory store intended for tests and ephemeral integrations.
#[derive(Debug)]
pub struct InMemorySessionStore<E = std::convert::Infallible> {
    seq_numbers: HashMap<SessionKey, SeqNumbers>,
    outbound_app: HashMap<SessionKey, BTreeMap<u64, Vec<u8>>>,
    marker: PhantomData<fn() -> E>,
}

impl<E> Default for InMemorySessionStore<E> {
    fn default() -> Self {
        Self {
            seq_numbers: HashMap::new(),
            outbound_app: HashMap::new(),
            marker: PhantomData,
        }
    }
}

#[async_trait]
impl<E> SessionStore for InMemorySessionStore<E>
where
    E: Send + Sync + 'static,
{
    type Error = E;

    async fn load_seq_numbers(
        &mut self,
        key: &SessionKey,
    ) -> Result<Option<SeqNumbers>, Self::Error> {
        Ok(self.seq_numbers.get(key).copied())
    }

    async fn save_seq_numbers(
        &mut self,
        key: &SessionKey,
        seq: SeqNumbers,
    ) -> Result<(), Self::Error> {
        self.seq_numbers.insert(key.clone(), seq);
        Ok(())
    }

    async fn save_outbound_app(
        &mut self,
        key: &SessionKey,
        msg: StoredAppMessage,
    ) -> Result<(), Self::Error> {
        self.outbound_app
            .entry(key.clone())
            .or_default()
            .insert(msg.seq_num, msg.raw_message);
        Ok(())
    }

    async fn load_outbound_app_range(
        &mut self,
        key: &SessionKey,
        start_inclusive: u64,
        end_inclusive: u64,
    ) -> Result<Vec<StoredAppMessage>, Self::Error> {
        if start_inclusive > end_inclusive {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        if let Some(messages) = self.outbound_app.get(key) {
            for (&seq_num, raw_message) in messages.range(start_inclusive..=end_inclusive) {
                out.push(StoredAppMessage {
                    seq_num,
                    raw_message: raw_message.clone(),
                });
            }
        }
        Ok(out)
    }

    async fn reset_session(
        &mut self,
        key: &SessionKey,
        seq: SeqNumbers,
    ) -> Result<(), Self::Error> {
        self.seq_numbers.insert(key.clone(), seq);
        self.outbound_app.remove(key);
        Ok(())
    }
}

#[cfg(feature = "session-store-sqlite")]
const SQLITE_SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS session_state (
    begin_string BLOB NOT NULL,
    sender_comp_id BLOB NOT NULL,
    target_comp_id BLOB NOT NULL,
    next_inbound INTEGER NOT NULL,
    next_outbound INTEGER NOT NULL,
    PRIMARY KEY (begin_string, sender_comp_id, target_comp_id)
);

CREATE TABLE IF NOT EXISTS outbound_app_messages (
    begin_string BLOB NOT NULL,
    sender_comp_id BLOB NOT NULL,
    target_comp_id BLOB NOT NULL,
    seq_num INTEGER NOT NULL,
    raw_message BLOB NOT NULL,
    PRIMARY KEY (begin_string, sender_comp_id, target_comp_id, seq_num)
);

CREATE INDEX IF NOT EXISTS idx_outbound_range
ON outbound_app_messages (begin_string, sender_comp_id, target_comp_id, seq_num);
"#;

#[cfg(feature = "session-store-sqlite")]
fn sqlite_io_error(operation: &str, error: rusqlite::Error) -> io::Error {
    io::Error::other(format!("sqlite {operation} failed: {error}"))
}

#[cfg(feature = "session-store-sqlite")]
fn encode_sqlite_seq(value: u64, context: &str) -> Result<i64, io::Error> {
    let value = i64::try_from(value).map_err(|_| {
        io::Error::other(format!(
            "{context}: value {value} exceeds sqlite INTEGER range"
        ))
    })?;
    if value <= 0 {
        return Err(io::Error::other(format!(
            "{context}: non-positive sequence value {value}"
        )));
    }
    Ok(value)
}

#[cfg(feature = "session-store-sqlite")]
fn decode_sqlite_seq(value: i64, context: &str) -> Result<u64, io::Error> {
    if value <= 0 {
        return Err(io::Error::other(format!(
            "{context}: non-positive sequence value {value}"
        )));
    }
    u64::try_from(value).map_err(|_| {
        io::Error::other(format!(
            "{context}: value {value} cannot be represented as u64"
        ))
    })
}

/// Configuration knobs for [`SqliteSessionStore`].
#[cfg(feature = "session-store-sqlite")]
#[derive(Debug, Clone)]
pub struct SqliteStoreOptions {
    /// How long sqlite should wait for busy locks before failing.
    pub busy_timeout: Duration,
    /// Enables `PRAGMA journal_mode = WAL` when true, otherwise `DELETE`.
    pub journal_mode_wal: bool,
    /// Enables `PRAGMA synchronous = FULL` when true, otherwise `NORMAL`.
    pub synchronous_full: bool,
}

#[cfg(feature = "session-store-sqlite")]
impl Default for SqliteStoreOptions {
    fn default() -> Self {
        Self {
            busy_timeout: Duration::from_secs(5),
            journal_mode_wal: true,
            synchronous_full: true,
        }
    }
}

/// SQLite-backed [`SessionStore`] implementation with durable replay state.
///
/// SQLite operations execute synchronously inside async methods to keep
/// `SessionStore` runtime-agnostic.
#[cfg(feature = "session-store-sqlite")]
#[derive(Debug)]
pub struct SqliteSessionStore {
    conn: rusqlite::Connection,
}

#[cfg(feature = "session-store-sqlite")]
impl SqliteSessionStore {
    /// Opens (or creates) a sqlite store with default durability settings.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        Self::open_with_options(path, SqliteStoreOptions::default())
    }

    /// Opens (or creates) a sqlite store with custom sqlite pragmas.
    pub fn open_with_options(
        path: impl AsRef<Path>,
        options: SqliteStoreOptions,
    ) -> Result<Self, io::Error> {
        let mut conn = rusqlite::Connection::open(path.as_ref())
            .map_err(|err| sqlite_io_error("open", err))?;
        Self::configure_connection(&mut conn, &options)?;
        Self::init_schema(&mut conn)?;
        Ok(Self { conn })
    }

    fn configure_connection(
        conn: &mut rusqlite::Connection,
        options: &SqliteStoreOptions,
    ) -> Result<(), io::Error> {
        let journal_mode = if options.journal_mode_wal {
            "WAL"
        } else {
            "DELETE"
        };
        conn.pragma_update(None, "journal_mode", journal_mode)
            .map_err(|err| sqlite_io_error("set pragma journal_mode", err))?;

        let synchronous = if options.synchronous_full {
            "FULL"
        } else {
            "NORMAL"
        };
        conn.pragma_update(None, "synchronous", synchronous)
            .map_err(|err| sqlite_io_error("set pragma synchronous", err))?;

        conn.busy_timeout(options.busy_timeout)
            .map_err(|err| sqlite_io_error("set busy_timeout", err))?;

        Ok(())
    }

    fn init_schema(conn: &mut rusqlite::Connection) -> Result<(), io::Error> {
        conn.execute_batch(SQLITE_SCHEMA_SQL)
            .map_err(|err| sqlite_io_error("init schema", err))
    }
}

#[cfg(feature = "session-store-sqlite")]
#[async_trait]
impl SessionStore for SqliteSessionStore {
    type Error = io::Error;

    async fn load_seq_numbers(
        &mut self,
        key: &SessionKey,
    ) -> Result<Option<SeqNumbers>, Self::Error> {
        use rusqlite::OptionalExtension;

        let row = self
            .conn
            .query_row(
                "SELECT next_inbound, next_outbound
                 FROM session_state
                 WHERE begin_string = ?1 AND sender_comp_id = ?2 AND target_comp_id = ?3",
                rusqlite::params![&key.begin_string, &key.sender_comp_id, &key.target_comp_id],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
            )
            .optional()
            .map_err(|err| sqlite_io_error("load_seq_numbers query", err))?;

        let Some((next_inbound, next_outbound)) = row else {
            return Ok(None);
        };

        Ok(Some(SeqNumbers {
            next_inbound: decode_sqlite_seq(next_inbound, "load_seq_numbers/next_inbound")?,
            next_outbound: decode_sqlite_seq(next_outbound, "load_seq_numbers/next_outbound")?,
        }))
    }

    async fn save_seq_numbers(
        &mut self,
        key: &SessionKey,
        seq: SeqNumbers,
    ) -> Result<(), Self::Error> {
        let next_inbound = encode_sqlite_seq(seq.next_inbound, "save_seq_numbers/next_inbound")?;
        let next_outbound = encode_sqlite_seq(seq.next_outbound, "save_seq_numbers/next_outbound")?;
        self.conn
            .execute(
                "INSERT INTO session_state
                 (begin_string, sender_comp_id, target_comp_id, next_inbound, next_outbound)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(begin_string, sender_comp_id, target_comp_id)
                 DO UPDATE SET
                   next_inbound = excluded.next_inbound,
                   next_outbound = excluded.next_outbound",
                rusqlite::params![
                    &key.begin_string,
                    &key.sender_comp_id,
                    &key.target_comp_id,
                    next_inbound,
                    next_outbound
                ],
            )
            .map_err(|err| sqlite_io_error("save_seq_numbers", err))?;
        Ok(())
    }

    async fn save_outbound_app(
        &mut self,
        key: &SessionKey,
        msg: StoredAppMessage,
    ) -> Result<(), Self::Error> {
        let seq_num = encode_sqlite_seq(msg.seq_num, "save_outbound_app/seq_num")?;
        self.conn
            .execute(
                "INSERT OR REPLACE INTO outbound_app_messages
                 (begin_string, sender_comp_id, target_comp_id, seq_num, raw_message)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![
                    &key.begin_string,
                    &key.sender_comp_id,
                    &key.target_comp_id,
                    seq_num,
                    msg.raw_message
                ],
            )
            .map_err(|err| sqlite_io_error("save_outbound_app", err))?;
        Ok(())
    }

    async fn load_outbound_app_range(
        &mut self,
        key: &SessionKey,
        start_inclusive: u64,
        end_inclusive: u64,
    ) -> Result<Vec<StoredAppMessage>, Self::Error> {
        if start_inclusive > end_inclusive {
            return Ok(Vec::new());
        }
        let start = encode_sqlite_seq(start_inclusive, "load_outbound_app_range/start_inclusive")?;
        let end = encode_sqlite_seq(end_inclusive, "load_outbound_app_range/end_inclusive")?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT seq_num, raw_message
                 FROM outbound_app_messages
                 WHERE begin_string = ?1
                   AND sender_comp_id = ?2
                   AND target_comp_id = ?3
                   AND seq_num BETWEEN ?4 AND ?5
                 ORDER BY seq_num ASC",
            )
            .map_err(|err| sqlite_io_error("load_outbound_app_range/prepare", err))?;
        let mut rows = stmt
            .query(rusqlite::params![
                &key.begin_string,
                &key.sender_comp_id,
                &key.target_comp_id,
                start,
                end
            ])
            .map_err(|err| sqlite_io_error("load_outbound_app_range/query", err))?;

        let mut messages = Vec::new();
        while let Some(row) = rows
            .next()
            .map_err(|err| sqlite_io_error("load_outbound_app_range/next", err))?
        {
            let seq_num = row
                .get::<_, i64>(0)
                .map_err(|err| sqlite_io_error("load_outbound_app_range/get seq_num", err))?;
            let raw_message = row
                .get::<_, Vec<u8>>(1)
                .map_err(|err| sqlite_io_error("load_outbound_app_range/get raw_message", err))?;
            messages.push(StoredAppMessage {
                seq_num: decode_sqlite_seq(seq_num, "load_outbound_app_range/seq_num")?,
                raw_message,
            });
        }

        Ok(messages)
    }

    async fn reset_session(
        &mut self,
        key: &SessionKey,
        seq: SeqNumbers,
    ) -> Result<(), Self::Error> {
        let next_inbound = encode_sqlite_seq(seq.next_inbound, "reset_session/next_inbound")?;
        let next_outbound = encode_sqlite_seq(seq.next_outbound, "reset_session/next_outbound")?;
        let tx = self
            .conn
            .transaction()
            .map_err(|err| sqlite_io_error("reset_session/begin_tx", err))?;

        tx.execute(
            "INSERT INTO session_state
             (begin_string, sender_comp_id, target_comp_id, next_inbound, next_outbound)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(begin_string, sender_comp_id, target_comp_id)
             DO UPDATE SET
               next_inbound = excluded.next_inbound,
               next_outbound = excluded.next_outbound",
            rusqlite::params![
                &key.begin_string,
                &key.sender_comp_id,
                &key.target_comp_id,
                next_inbound,
                next_outbound
            ],
        )
        .map_err(|err| sqlite_io_error("reset_session/save_seq_numbers", err))?;

        tx.execute(
            "DELETE FROM outbound_app_messages
             WHERE begin_string = ?1 AND sender_comp_id = ?2 AND target_comp_id = ?3",
            rusqlite::params![&key.begin_string, &key.sender_comp_id, &key.target_comp_id],
        )
        .map_err(|err| sqlite_io_error("reset_session/delete_outbound", err))?;

        tx.commit()
            .map_err(|err| sqlite_io_error("reset_session/commit", err))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::executor::block_on;
    use std::num::NonZeroU64;
    #[cfg(feature = "session-store-sqlite")]
    use std::{path::Path, path::PathBuf};
    #[cfg(feature = "session-store-sqlite")]
    use uuid::Uuid;

    fn key() -> SessionKey {
        SessionKey {
            begin_string: b"FIX.4.4".to_vec(),
            sender_comp_id: b"S".to_vec(),
            target_comp_id: b"T".to_vec(),
        }
    }

    #[cfg(feature = "session-store-sqlite")]
    fn key_with(sender: &[u8], target: &[u8]) -> SessionKey {
        SessionKey {
            begin_string: b"FIX.4.4".to_vec(),
            sender_comp_id: sender.to_vec(),
            target_comp_id: target.to_vec(),
        }
    }

    #[cfg(feature = "session-store-sqlite")]
    struct TempSqlitePath {
        path: PathBuf,
    }

    #[cfg(feature = "session-store-sqlite")]
    impl TempSqlitePath {
        fn new() -> Self {
            let path = std::env::temp_dir()
                .join(format!("fefix-session-store-{}.sqlite3", Uuid::new_v4()));
            Self { path }
        }

        fn as_path(&self) -> &Path {
            &self.path
        }
    }

    #[cfg(feature = "session-store-sqlite")]
    impl Drop for TempSqlitePath {
        fn drop(&mut self) {
            for suffix in ["", "-wal", "-shm"] {
                let candidate = if suffix.is_empty() {
                    self.path.clone()
                } else {
                    PathBuf::from(format!("{}{}", self.path.display(), suffix))
                };
                let _ = std::fs::remove_file(candidate);
            }
        }
    }

    #[test]
    fn in_memory_store_round_trip() {
        let mut store = InMemorySessionStore::<()>::default();
        let session = key();
        let seq = SeqNumbers::new(NonZeroU64::new(7).unwrap(), NonZeroU64::new(9).unwrap());

        block_on(store.save_seq_numbers(&session, seq)).unwrap();
        let loaded = block_on(store.load_seq_numbers(&session)).unwrap().unwrap();
        assert_eq!(loaded.next_inbound(), 7);
        assert_eq!(loaded.next_outbound(), 9);

        block_on(store.save_outbound_app(
            &session,
            StoredAppMessage {
                seq_num: 3,
                raw_message: b"8=FIX.4.4\x019=5\x0135=D\x0134=3\x0110=000\x01".to_vec(),
            },
        ))
        .unwrap();
        let replay = block_on(store.load_outbound_app_range(&session, 1, 10)).unwrap();
        assert_eq!(replay.len(), 1);
        assert_eq!(replay[0].seq_num, 3);

        block_on(store.reset_session(&session, SeqNumbers::default())).unwrap();
        assert!(block_on(store.load_outbound_app_range(&session, 1, 10))
            .unwrap()
            .is_empty());
        let after_reset = block_on(store.load_seq_numbers(&session)).unwrap().unwrap();
        assert_eq!(after_reset.next_inbound(), 1);
        assert_eq!(after_reset.next_outbound(), 1);
    }

    #[cfg(feature = "session-store-sqlite")]
    #[test]
    fn sqlite_store_creates_schema_on_open() {
        let path = TempSqlitePath::new();
        let _store = SqliteSessionStore::open(path.as_path()).unwrap();

        let conn = rusqlite::Connection::open(path.as_path()).unwrap();
        let session_state_count: i64 = conn
            .query_row(
                "SELECT COUNT(1) FROM sqlite_master WHERE type = 'table' AND name = 'session_state'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(session_state_count, 1);

        let outbound_table_count: i64 = conn
            .query_row(
                "SELECT COUNT(1) FROM sqlite_master WHERE type = 'table' AND name = 'outbound_app_messages'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(outbound_table_count, 1);

        let index_count: i64 = conn
            .query_row(
                "SELECT COUNT(1) FROM sqlite_master WHERE type = 'index' AND name = 'idx_outbound_range'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(index_count, 1);
    }

    #[cfg(feature = "session-store-sqlite")]
    #[test]
    fn sqlite_seq_numbers_round_trip_across_reopen() {
        let path = TempSqlitePath::new();
        let session_key = key();
        let seq = SeqNumbers::new(NonZeroU64::new(7).unwrap(), NonZeroU64::new(9).unwrap());
        {
            let mut store = SqliteSessionStore::open(path.as_path()).unwrap();
            block_on(store.save_seq_numbers(&session_key, seq)).unwrap();
        }
        let mut reopened = SqliteSessionStore::open(path.as_path()).unwrap();
        let loaded = block_on(reopened.load_seq_numbers(&session_key))
            .unwrap()
            .unwrap();
        assert_eq!(loaded.next_inbound(), 7);
        assert_eq!(loaded.next_outbound(), 9);
    }

    #[cfg(feature = "session-store-sqlite")]
    #[test]
    fn sqlite_outbound_app_round_trip_and_ordering() {
        let path = TempSqlitePath::new();
        let session_key = key();
        let mut store = SqliteSessionStore::open(path.as_path()).unwrap();
        for seq in [3_u64, 1_u64, 2_u64] {
            block_on(store.save_outbound_app(
                &session_key,
                StoredAppMessage {
                    seq_num: seq,
                    raw_message: format!("msg-{seq}").into_bytes(),
                },
            ))
            .unwrap();
        }
        let loaded = block_on(store.load_outbound_app_range(&session_key, 1, 3)).unwrap();
        assert_eq!(loaded.len(), 3);
        assert_eq!(loaded[0].seq_num, 1);
        assert_eq!(loaded[1].seq_num, 2);
        assert_eq!(loaded[2].seq_num, 3);
    }

    #[cfg(feature = "session-store-sqlite")]
    #[test]
    fn sqlite_load_outbound_range_respects_bounds() {
        let path = TempSqlitePath::new();
        let session_key = key();
        let mut store = SqliteSessionStore::open(path.as_path()).unwrap();
        for seq in 1_u64..=5_u64 {
            block_on(store.save_outbound_app(
                &session_key,
                StoredAppMessage {
                    seq_num: seq,
                    raw_message: format!("msg-{seq}").into_bytes(),
                },
            ))
            .unwrap();
        }
        let loaded = block_on(store.load_outbound_app_range(&session_key, 2, 4)).unwrap();
        assert_eq!(loaded.len(), 3);
        assert_eq!(loaded[0].seq_num, 2);
        assert_eq!(loaded[1].seq_num, 3);
        assert_eq!(loaded[2].seq_num, 4);
    }

    #[cfg(feature = "session-store-sqlite")]
    #[test]
    fn sqlite_reset_session_clears_outbound_and_sets_seq() {
        let path = TempSqlitePath::new();
        let session_key = key();
        let mut store = SqliteSessionStore::open(path.as_path()).unwrap();
        block_on(store.save_seq_numbers(
            &session_key,
            SeqNumbers::new(NonZeroU64::new(11).unwrap(), NonZeroU64::new(13).unwrap()),
        ))
        .unwrap();
        block_on(store.save_outbound_app(
            &session_key,
            StoredAppMessage {
                seq_num: 11,
                raw_message: b"msg-11".to_vec(),
            },
        ))
        .unwrap();
        block_on(store.reset_session(&session_key, SeqNumbers::default())).unwrap();

        let loaded_seq = block_on(store.load_seq_numbers(&session_key))
            .unwrap()
            .unwrap();
        assert_eq!(loaded_seq.next_inbound(), 1);
        assert_eq!(loaded_seq.next_outbound(), 1);
        assert!(block_on(store.load_outbound_app_range(&session_key, 1, 99))
            .unwrap()
            .is_empty());
    }

    #[cfg(feature = "session-store-sqlite")]
    #[test]
    fn sqlite_sessions_are_isolated_by_session_key() {
        let path = TempSqlitePath::new();
        let key_a = key_with(b"S-A", b"T-A");
        let key_b = key_with(b"S-B", b"T-B");
        let mut store = SqliteSessionStore::open(path.as_path()).unwrap();

        block_on(store.save_seq_numbers(
            &key_a,
            SeqNumbers::new(NonZeroU64::new(2).unwrap(), NonZeroU64::new(3).unwrap()),
        ))
        .unwrap();
        block_on(store.save_seq_numbers(
            &key_b,
            SeqNumbers::new(NonZeroU64::new(5).unwrap(), NonZeroU64::new(6).unwrap()),
        ))
        .unwrap();

        block_on(store.save_outbound_app(
            &key_a,
            StoredAppMessage {
                seq_num: 2,
                raw_message: b"a".to_vec(),
            },
        ))
        .unwrap();
        block_on(store.save_outbound_app(
            &key_b,
            StoredAppMessage {
                seq_num: 5,
                raw_message: b"b".to_vec(),
            },
        ))
        .unwrap();

        let seq_a = block_on(store.load_seq_numbers(&key_a)).unwrap().unwrap();
        let seq_b = block_on(store.load_seq_numbers(&key_b)).unwrap().unwrap();
        assert_eq!(seq_a.next_outbound(), 3);
        assert_eq!(seq_b.next_outbound(), 6);

        let app_a = block_on(store.load_outbound_app_range(&key_a, 1, 10)).unwrap();
        let app_b = block_on(store.load_outbound_app_range(&key_b, 1, 10)).unwrap();
        assert_eq!(app_a.len(), 1);
        assert_eq!(app_b.len(), 1);
        assert_eq!(app_a[0].raw_message, b"a");
        assert_eq!(app_b[0].raw_message, b"b");
    }

    #[cfg(feature = "session-store-sqlite")]
    #[test]
    fn sqlite_store_persists_after_process_restart_simulation() {
        let path = TempSqlitePath::new();
        let session_key = key();
        {
            let mut store = SqliteSessionStore::open(path.as_path()).unwrap();
            block_on(store.save_seq_numbers(
                &session_key,
                SeqNumbers::new(NonZeroU64::new(8).unwrap(), NonZeroU64::new(10).unwrap()),
            ))
            .unwrap();
            block_on(store.save_outbound_app(
                &session_key,
                StoredAppMessage {
                    seq_num: 9,
                    raw_message: b"persist-me".to_vec(),
                },
            ))
            .unwrap();
        }
        let mut reopened = SqliteSessionStore::open(path.as_path()).unwrap();
        let loaded_seq = block_on(reopened.load_seq_numbers(&session_key))
            .unwrap()
            .unwrap();
        assert_eq!(loaded_seq.next_inbound(), 8);
        assert_eq!(loaded_seq.next_outbound(), 10);

        let loaded = block_on(reopened.load_outbound_app_range(&session_key, 1, 20)).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].seq_num, 9);
        assert_eq!(loaded[0].raw_message, b"persist-me");
    }
}
