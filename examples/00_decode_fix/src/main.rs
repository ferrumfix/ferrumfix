use rustyfix::prelude::*;
use rustyfix::tagvalue::Decoder;

const FIX_MESSAGE: &[u8] = b"8=FIX.4.2|9=196|35=X|49=A|56=B|34=12|52=20100318-03:21:11.364|262=A|268=2|279=0|269=0|278=BID|55=EUR/USD|270=1.37215|15=EUR|271=2500000|346=1|279=0|269=1|278=OFFER|55=EUR/USD|270=1.37224|15=EUR|271=2503200|346=1|10=171|";

fn main() {
    let fix_dictionary = Dictionary::fix42().expect("Failed to load FIX 4.2 dictionary");
    // Let's create a FIX decoder. This is an expensive operation, and it should
    // only be done once at the beginning of your program and/or FIX session.
    let mut fix_decoder = Decoder::new(fix_dictionary);
    // In this case, the FIX message is specified using "|" rather than SOH
    // (ASCII 0x1) bytes. FerrumFIX supports this.
    fix_decoder.config_mut().separator = b'|';
    let msg = fix_decoder
        .decode(FIX_MESSAGE)
        .expect("Invalid FIX message");

    // Read the FIX message! You get nice type inference out of the box. You
    // have fine-grained control over how to decode each field, even down to raw
    // bytes if you want full control.
    assert_eq!(msg.get(fix42::BEGIN_STRING.tag), Ok("FIX.4.2"));
    assert_eq!(msg.get(fix42::MSG_TYPE.tag), Ok("X"));
    assert_eq!(
        msg.get(fix42::MSG_TYPE.tag),
        Ok(fix42::MsgType::MarketDataIncrementalRefresh)
    );
    assert_eq!(
        msg.get(fix42::MSG_TYPE.tag),
        Ok(fix42::MsgType::MarketDataIncrementalRefresh)
    );
    assert_eq!(msg.get(fix42::SENDER_COMP_ID.tag), Ok("A"));
    assert_eq!(msg.get(fix42::TARGET_COMP_ID.tag), Ok("B"));
    assert_eq!(msg.get(fix42::MSG_SEQ_NUM.tag), Ok(12));
    assert_eq!(msg.get(fix42::MD_REQ_ID.tag), Ok("A"));

    // Repeating groups!
    let md_entries = msg.group(fix42::NO_MD_ENTRIES.tag).unwrap();
    assert_eq!(md_entries.len(), 2);

    for entry in md_entries.entries() {
        assert_eq!(entry.get(fix42::CURRENCY.tag), Ok("EUR"));
    }

    let md0 = md_entries.get(0).unwrap();
    assert_eq!(
        md0.get(fix42::MD_UPDATE_ACTION.tag),
        Ok(fix42::MdUpdateAction::New)
    );
    assert_eq!(md0.get(fix42::MD_ENTRY_ID.tag), Ok("BID"));
    assert_eq!(md0.get(fix42::SYMBOL.tag), Ok("EUR/USD"));
    assert_eq!(md0.get(fix42::MD_ENTRY_PX.tag), Ok(1.37215f32));
    assert_eq!(md0.get(fix42::MD_ENTRY_PX.tag), Ok("1.37215"));
    assert_eq!(md0.get(fix42::CURRENCY.tag), Ok("EUR"));
    assert_eq!(md0.get(fix42::MD_ENTRY_SIZE.tag), Ok(2_500_000));
    assert_eq!(md0.get(fix42::NUMBER_OF_ORDERS.tag), Ok(1));

    let md1 = md_entries.get(1).unwrap();
    assert_eq!(
        md1.get(fix42::MD_UPDATE_ACTION.tag),
        Ok(fix42::MdUpdateAction::New)
    );
    assert_eq!(md1.get(fix42::MD_ENTRY_ID.tag), Ok("OFFER"));
    assert_eq!(md1.get(fix42::SYMBOL.tag), Ok("EUR/USD"));
    assert_eq!(md1.get(fix42::MD_ENTRY_PX.tag), Ok("1.37224"));
    assert_eq!(md1.get(fix42::MD_ENTRY_PX.tag), Ok(1.37224));
    assert_eq!(md1.get(fix42::CURRENCY.tag), Ok("EUR"));
    assert_eq!(md1.get(fix42::MD_ENTRY_SIZE.tag), Ok(2_503_200));
    assert_eq!(md1.get(fix42::NUMBER_OF_ORDERS.tag), Ok(1));

    // You can use either mnemonics (like above) or tag numbers for keys.
    assert_eq!(msg.get(49), Ok("A"));
}

#[cfg(test)]
#[test]
fn run() {
    main();
}
