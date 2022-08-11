use fefix::tagvalue::Decoder;
use fefix::{prelude::*, tagvalue::Config};
use std::io::{Cursor, Read};

const FIX_MESSAGES: &[&[u8]] = &[
    b"8=FIX.4.2|9=97|35=6|49=BKR|56=IM|34=14|52=20100204-09:18:42|23=115685|28=N|55=SPMI.MI|54=2|44=2200.75|27=S|25=H|10=248|",
    b"8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|",
];

fn fix_stream() -> Vec<u8> {
    FIX_MESSAGES.iter().copied().flatten().copied().collect()
}

/// See [`fix::tagvalue::decoder::test::decoder`]
fn decoder(fix_dictionary: Dictionary) -> Decoder<Config> {
    // Let's create a FIX decoder. This is an expensive operation, and it should
    // only be done once at the beginning of your program and/or FIX session.
    let mut decoder = Decoder::<Config>::new(fix_dictionary);
    // In this case, the FIX message is specified using "|" rather than SOH
    // (ASCII 0x1) bytes. FerrumFIX supports this.
    decoder.config_mut().set_separator(b'|');
    decoder
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fix_dictionary = Dictionary::fix42();
    let mut fix_decoder = decoder(fix_dictionary).streaming(vec![]);
    let mut stream = Cursor::new(fix_stream());
    for _message in 0..FIX_MESSAGES.len() {
        loop {
            // You *must* use `std::io::Read::read_exact`.
            stream.read_exact(fix_decoder.fillable())?;
            match fix_decoder.try_parse()? {
                Some(_) => {
                    // we have successfully parsed a message
                    let msg = fix_decoder.message();
                    assert_eq!(msg.fv(fix42::BEGIN_STRING), Ok("FIX.4.2"));
                    // need to clear the decoder to to begin parsing next mesage
                    fix_decoder.clear();
                    break;
                }
                None => {
                    println!("still-parsing-message");
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[test]
fn run() {
    main().unwrap();
}
