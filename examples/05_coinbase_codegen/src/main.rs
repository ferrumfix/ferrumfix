#[allow(dead_code)]
#[rustfmt::skip]
mod gdax;

use fefix::prelude::*;
use fefix::tagvalue::Decoder;

const QUICKFIX_SPEC: &str = include_str!("coinbase_quickfix.xml");

const FIX_MESSAGE_EXEC_REPORT: &[u8] = b"8=FIX.4.2|9=21|35=8|1003=123|1057=Y|10=090|";

fn main() {
    let mut decoder = fix_decoder();
    decoder.config_mut().separator = b'|';
    let msg = decoder
        .decode(FIX_MESSAGE_EXEC_REPORT)
        .expect("Invalid FIX message");
    assert_eq!(msg.fv(gdax::BEGIN_STRING), Ok(gdax::BeginString::Fix42));
    assert_eq!(msg.fv(gdax::MSG_TYPE), Ok(gdax::MsgType::ExecutionReport));
    assert_eq!(msg.fv(gdax::TRADE_ID), Ok("123"));
    assert_eq!(msg.fv(gdax::AGGRESSOR_INDICATOR), Ok(true));
}

fn fix_decoder() -> Decoder {
    let fix_dictionary = Dictionary::from_quickfix_spec(QUICKFIX_SPEC).unwrap();
    Decoder::new(fix_dictionary)
}

#[cfg(test)]
#[test]
fn run() {
    main();
}
