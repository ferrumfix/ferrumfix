use fefix::fields::fix42;
use fefix::tagvalue::{Config, Decoder};
use fefix::{AppVersion, Dictionary};

const FIX_MESSAGE: &[u8] = b"8=FIX.4.2|9=97|35=6|49=BKR|56=IM|34=14|52=20100204-09:18:42|23=115685|28=N|55=SPMI.MI|54=2|44=2200.75|27=S|25=H|10=248|";

fn main() {
    let fix_dictionary = Dictionary::from_version(AppVersion::Fix42);
    let fix_decoder = &mut Decoder::<Config>::new(fix_dictionary);
    let message = fix_decoder
        .decode(FIX_MESSAGE)
        .expect("Invalid FIX message");
    assert_eq!(message.field_ref(fix42::BEGIN_STRING), Ok(b"FIX" as &[u8]));
    assert_eq!(
        message.field_ref(fix42::MSG_TYPE),
        Ok(fix42::MsgType::IndicationOfInterest)
    );
    assert_eq!(
        message.field_ref(fix42::SENDER_COMP_ID),
        Ok(b"BKR" as &[u8])
    );
    assert_eq!(message.field_ref(fix42::TARGET_COMP_ID), Ok(b"IM" as &[u8]));
    assert_eq!(message.field_ref(fix42::MSG_SEQ_NUM), Ok(14));
    assert_eq!(message.field_ref(fix42::IO_IID), Ok(b"115685" as &[u8]));
    assert_eq!(
        message.field_ref(fix42::IOI_TRANS_TYPE),
        Ok(fix42::IoitransType::New)
    );
    assert_eq!(message.field_ref(fix42::SYMBOL), Ok(b"SPMI.MI" as &[u8]));
    assert_eq!(message.field_ref(fix42::SIDE), Ok(fix42::Side::Sell));
    println!("Hello, world!");
}
