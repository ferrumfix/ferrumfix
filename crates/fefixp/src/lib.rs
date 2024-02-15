//! *FIX Performance Session Layer*
//! ([FIXP](https://www.fixtrading.org/standards/fixp-online/)) support.

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![allow(dead_code)]

type SessionId = u128;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlowType {
    Recoverable,
    Idempotent,
    Unsequenced,
    None,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MessageType {
    Sequence,
    Context,
    MessageTemplate,
    Negotiate,
}

#[derive(Debug, Clone)]
pub struct Sequence {
    next_seq_number: u64,
}

#[derive(Debug, Clone)]
pub struct Context {
    session_id: SessionId,
    next_seq_number: u64,
}

#[derive(Debug, Clone)]
pub struct MessageTemplate {
    encoding_type: u32,
    effective_time: u64,
    version: Vec<u8>,
    template: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Negotiate {
    session_id: SessionId,
    client_flow: FlowType,
    credentials: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct NegotiationReject {
    session_id: SessionId,
    reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Establish {
    session_id: SessionId,
    next_seq_number: u64,
    credentials: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct EstablishmentAck {
    next_seq_number: u64,
}
