use fefix::definitions::fix42;
use fefix::tagvalue::{Config, Decoder, Fv};
use fefix::Dictionary;
use std::io::{Cursor, Read};

const FIX_MESSAGES: &[&[u8]] = &[
    b"8=FIX.4.2|9=97|35=6|49=BKR|56=IM|34=14|52=20100204-09:18:42|23=115685|28=N|55=SPMI.MI|54=2|44=2200.75|27=S|25=H|10=248|",
];

fn fix_stream() -> Vec<u8> {
    FIX_MESSAGES.iter().copied().flatten().copied().collect()
}

fn main() {
    let fix_dictionary = Dictionary::fix42();
    // Let's create a FIX decoder. This is an expensive operation, and it should
    // only be done once at the beginning of your program and/or FIX session.
    let fix_decoder = &mut Decoder::<Config>::new(fix_dictionary).buffered();
    // In this case, the FIX message is specified using "|" rather than SOH
    // (ASCII 0x1) bytes. FerrumFIX supports this.
    fix_decoder.config_mut().set_separator(b'|');
    fix_decoder.config_mut().set_verify_checksum(false);
    let stream = &mut Cursor::new(fix_stream());
    loop {
        let buffer = &mut fix_decoder.supply_buffer();
        // You *must* use `std::io::Read::read_exact`.
        stream.read_exact(buffer).unwrap();
        if let Ok(Some(msg)) = fix_decoder.current_message() {
            assert_eq!(msg.fv(fix42::BEGIN_STRING), Ok("FIX.4.2"));
        }
    }
}
