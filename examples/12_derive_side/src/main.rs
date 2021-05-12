use fefix::prelude::*;
use fefix::tagvalue::{Config, Decoder};
use fefix::FixValue;

const FIX_MESSAGE: &[u8] = b"8=FIX.4.2|9=97|35=6|49=BKR|56=IM|34=14|52=20100204-09:18:42|23=115685|28=N|55=SPMI.MI|54=2|44=2200.75|27=S|25=H|10=248|";

fn main() {
    let fix_dictionary = Dictionary::fix42();
    let fix_decoder = &mut Decoder::<Config>::new(fix_dictionary);
    fix_decoder.config_mut().set_separator(b'|');
    fix_decoder.config_mut().set_verify_checksum(false);
    let msg = fix_decoder
        .decode(FIX_MESSAGE)
        .expect("Invalid FIX message");
    assert_eq!(msg.fv(fix42::SIDE), Ok(Side::Buy));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, FixValue)]
pub enum Side {
    #[fefix(variant = "1")]
    Buy,
    #[fefix(variant = "2")]
    Sell,
}
