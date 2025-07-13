//! Generators for common `Text (58) values`.

#![allow(dead_code)]

/// Enhanced error message for heartbeat validation with optional raw message context.
pub fn heartbeat_exact_with_context(secs: u64, raw_message: Option<&[u8]>) -> String {
    let base_msg = format!("Invalid HeartBtInt(108), expected value {secs} seconds");
    if let Some(raw) = raw_message {
        format!("{base_msg} (raw message: {})", hex_debug(raw))
    } else {
        base_msg
    }
}

pub fn heartbeat_exact(secs: u64) -> String {
    heartbeat_exact_with_context(secs, None)
}

/// Enhanced error message for heartbeat range validation with optional raw message context.
pub fn heartbeat_range_with_context(a: u64, b: u64, raw_message: Option<&[u8]>) -> String {
    let base_msg = format!("Invalid HeartBtInt(108), expected value between {a} and {b} seconds");
    if let Some(raw) = raw_message {
        format!("{base_msg} (raw message: {})", hex_debug(raw))
    } else {
        base_msg
    }
}

pub fn heartbeat_range(a: u64, b: u64) -> String {
    heartbeat_range_with_context(a, b, None)
}

/// Enhanced error message for heartbeat > 0 validation with optional raw message context.
pub fn heartbeat_gt_0_with_context(raw_message: Option<&[u8]>) -> String {
    let base_msg = "Invalid HeartBtInt(108), expected value greater than 0 seconds";
    if let Some(raw) = raw_message {
        format!("{base_msg} (raw message: {})", hex_debug(raw))
    } else {
        base_msg.to_string()
    }
}

pub fn heartbeat_gt_0() -> String {
    heartbeat_gt_0_with_context(None)
}

pub fn inbound_seqnum() -> String {
    "NextExpectedMsgSeqNum(789) > than last message sent".to_string()
}

/// Enhanced error message for sequence number validation with optional raw message context.
pub fn msg_seq_num_with_context(seq_number: u64, raw_message: Option<&[u8]>) -> String {
    let base_msg = format!("Invalid MsgSeqNum <34>, expected value {seq_number}");
    if let Some(raw) = raw_message {
        format!("{base_msg} (raw message: {})", hex_debug(raw))
    } else {
        base_msg
    }
}

pub fn msg_seq_num(seq_number: u64) -> String {
    msg_seq_num_with_context(seq_number, None)
}

/// Enhanced error message for production environment validation with optional raw message context.
pub fn production_env_with_context(raw_message: Option<&[u8]>) -> String {
    let base_msg =
        "TestMessageIndicator(464) was set to 'Y' but the environment is a production environment";
    if let Some(raw) = raw_message {
        format!("{base_msg} (raw message: {})", hex_debug(raw))
    } else {
        base_msg.to_string()
    }
}

pub fn production_env() -> String {
    production_env_with_context(None)
}

/// Enhanced error message for missing fields with optional raw message context.
pub fn missing_field_with_context(name: &str, tag: u32, raw_message: Option<&[u8]>) -> String {
    let base_msg = format!("Missing mandatory field {name}({tag})");
    if let Some(raw) = raw_message {
        format!("{base_msg} (raw message: {})", hex_debug(raw))
    } else {
        base_msg
    }
}

pub fn missing_field(name: &str, tag: u32) -> String {
    missing_field_with_context(name, tag, None)
}

/// Utility function to create a hex debug representation of raw message bytes.
/// Truncates long messages and shows printable characters when possible.
fn hex_debug(raw: &[u8]) -> String {
    const MAX_LEN: usize = 64; // Limit output length for readability

    if raw.len() <= MAX_LEN {
        // For short messages, show both hex and ASCII representation
        let hex = raw
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ");

        let ascii = raw
            .iter()
            .map(|&b| {
                if b.is_ascii_graphic() || b == b' ' {
                    b as char
                } else {
                    '.'
                }
            })
            .collect::<String>();

        format!("0x[{hex}] \"{ascii}\"")
    } else {
        // For long messages, truncate and show length
        let truncated = &raw[..MAX_LEN];
        let hex = truncated
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            "0x[{}...] (length: {} bytes, showing first {})",
            hex,
            raw.len(),
            MAX_LEN
        )
    }
}
