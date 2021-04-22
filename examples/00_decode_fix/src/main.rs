use fefix::fix42;
use fefix::tagvalue::{Config, Decoder, Fv};
use fefix::{AppVersion, Dictionary};

const FIX_MESSAGE: &[u8] = b"8=FIX.4.2|9=97|35=6|49=BKR|56=IM|34=14|52=20100204-09:18:42|23=115685|28=N|55=SPMI.MI|54=2|44=2200.75|27=S|25=H|10=248|";

fn main() {
    let fix_dictionary = Dictionary::from_version(AppVersion::Fix42);
    // Let's create a FIX decoder. This is an expensive operation, and it should
    // only be done once at the beginning of your program and/or FIX session.
    let fix_decoder = &mut Decoder::<Config>::new(fix_dictionary);
    // In this case, the FIX message is specified using "|" rather than SOH
    // (ASCII 0x1) bytes. FerrumFIX supports this.
    fix_decoder.config_mut().set_separator(b'|');
    fix_decoder.config_mut().set_verify_checksum(false);
    let msg = fix_decoder
        .decode(FIX_MESSAGE)
        .expect("Invalid FIX message");
    // Read the FIX message!
    assert_eq!(msg.fv(fix42::BEGIN_STRING), Ok("FIX.4.2"));
    assert_eq!(
        msg.fv(fix42::MSG_TYPE),
        Ok(fix42::MsgType::IndicationOfInterest)
    );
    assert_eq!(msg.fv(fix42::SENDER_COMP_ID), Ok("BKR"));
    assert_eq!(msg.fv(fix42::TARGET_COMP_ID), Ok("IM"));
    assert_eq!(msg.fv(fix42::MSG_SEQ_NUM), Ok(14));
    assert_eq!(msg.fv(fix42::IO_IID), Ok("115685"));
    assert_eq!(msg.fv(fix42::IOI_TRANS_TYPE), Ok(fix42::IoiTransType::New));
    assert_eq!(msg.fv(fix42::SYMBOL), Ok("SPMI.MI"));
    assert_eq!(msg.fv(fix42::SIDE), Ok(fix42::Side::Sell));
    assert_eq!(msg.fv(fix42::IOI_QLTY_IND), Ok(fix42::IoiQltyInd::High));
}
