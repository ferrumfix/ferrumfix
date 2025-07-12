#[allow(dead_code)]
#[rustfmt::skip]
mod gdax;

use rustyfix::prelude::*;
use rustyfix::tagvalue::Decoder;

const QUICKFIX_SPEC: &str = include_str!("coinbase_quickfix.xml");

const FIX_MESSAGE_EXEC_REPORT: &[u8] = b"8=FIX.4.2|9=21|35=8|1003=123|1057=Y|10=090|";

fn main() {
    let mut buffer = Vec::new();
    let mut decoder = Decoder::new(gdax_dict.clone());
    let message = decoder
        .decode_frame(&mut buffer, &SAMPLE_MESSAGE[..])
        .unwrap();
    assert_eq!(
        message.get(gdax::BEGIN_STRING),
        Ok(gdax::BeginString::Fix42)
    );
    assert_eq!(
        message.get(gdax::MSG_TYPE),
        Ok(gdax::MsgType::ExecutionReport)
    );
    assert_eq!(message.get(gdax::TRADE_ID), Ok("123"));
    assert_eq!(message.get(gdax::AGGRESSOR_INDICATOR), Ok(true));
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
