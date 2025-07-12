//! Generators for common `Text (58) values`.

#![allow(dead_code)]

pub fn heartbeat_exact(secs: u64) -> String {
    format!("Invalid HeartBtInt(108), expected value {secs} seconds")
}

pub fn heartbeat_range(a: u64, b: u64) -> String {
    format!("Invalid HeartBtInt(108), expected value between {a} and {b} seconds",)
}

pub fn heartbeat_gt_0() -> String {
    "Invalid HeartBtInt(108), expected value greater than 0 seconds".to_string()
}

pub fn inbound_seqnum() -> String {
    "NextExpectedMsgSeqNum(789) > than last message sent".to_string()
}

pub fn msg_seq_num(seq_number: u64) -> String {
    format!("Invalid MsgSeqNum <34>, expected value {seq_number}")
}

pub fn production_env() -> String {
    "TestMessageIndicator(464) was set to 'Y' but the environment is a production environment"
        .to_string()
}

pub fn missing_field(name: &str, tag: u32) -> String {
    format!("Missing mandatory field {name}({tag})")
}
