use rustyfix::definitions::fix44;
use rustyfix::dict::IsFieldDefinition;
use rustyfix::prelude::*;
use rustyfix::tagvalue::Decoder;

#[test]
fn test_tainted_decoder_fix44_regression() {
    const SAMPLES: [&[u8]; 2] = [
        b"8=FIX.4.4\x019=176\x0135=X\x0149=ERISX\x0156=XXXXXXXXX\x0134=3\x01\
        52=20220714-09:26:22.518\x01262=TEST-220714092622-EfkcibvXPhF34SVNQYwwRz\x01\
        60=20220714-09:26:22.517712684\x016006=0\x01268=1\x01279=0\x01269=J\x01\
        55=BTC/USD\x0115=BTC\x0110=157\x01",
        b"8=FIX.4.4\x019=227\x0135=f\x0149=ERISX\x0156=XXXXXXXXX\x0134=4\x01\
        52=20220714-09:26:22.518\x01324=TEST-220714092622-EfkcibvXPhF34SVNQYwwRz\x01\
        60=20220714-09:26:22.517712684\x0155=BTC/USD\x01460=2\x01107=BTC/USD\x016006=0\x01\
        15=BTC\x01326=17\x01969=0.1\x01562=0.0001\x011140=100000\x01561=0.000001\x0110=098\x01",
    ];

    let mut decoder = Decoder::new(Dictionary::fix44().expect("Failed to load FIX 4.4 dictionary"));
    decoder.config_mut().verify_checksum = false;

    for sample in SAMPLES {
        let message = decoder
            .decode(sample)
            .expect("Couldn't decode sample FIX message");
        let msg_type = message.get::<fix44::MsgType>(fix44::MSG_TYPE.tag().get());
        assert!(msg_type.is_ok(), "fv() returns {msg_type:?}");
    }
}
