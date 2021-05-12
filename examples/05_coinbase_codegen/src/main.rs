#[allow(dead_code)]
#[rustfmt::skip]
mod gdax;

use fefix::prelude::*;
use fefix::tagvalue::{Config, Decoder};

const QUICKFIX_SPEC: &str = include_str!("coinbase_quickfix.xml");

const FIX_MESSAGE_EXEC_REPORT: &[u8] = b"";

fn main() {
    let decoder = &mut fix_decoder();
    decoder.config_mut().set_separator(b'|');
    decoder.config_mut().set_verify_checksum(false);
    let msg = decoder
        .decode(FIX_MESSAGE_EXEC_REPORT)
        .expect("Invalid FIX message");
    assert_eq!(msg.fv(gdax::BEGIN_STRING), Ok(gdax::BeginString::Fix42));
    assert_eq!(msg.fv(gdax::MSG_TYPE), Ok(gdax::MsgType::ExecutionReport));
    assert_eq!(msg.fv(gdax::TRADE_ID), Ok("123"));
    assert_eq!(msg.fv(gdax::AGGRESSOR_INDICATOR), Ok(true));
}

fn fix_decoder() -> Decoder<Config> {
    let fix_dictionary = Dictionary::from_quickfix_spec(QUICKFIX_SPEC).unwrap();
    Decoder::<Config>::new(fix_dictionary)
}
