use decimal::d128;
use fefix::prelude::*;
use fefix::tagvalue::{Config, Encoder, FvWrite};
use rust_decimal_macros::dec;

// 8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072|

fn main() {
    let encoder = &mut fix_encoder();
    let buffer = &mut Vec::new();
    let msg = &mut encoder.start_message(b"FIX.4.4", buffer, b"ExecutionReport");
    msg.set_fv(fix44::MSG_SEQ_NUM, 215);
    msg.set_fv(fix44::SENDER_COMP_ID, "CLIENT12");
    msg.set_fv(fix44::TARGET_COMP_ID, "B");
    msg.set_fv(fix44::ACCOUNT, "Marcel");
    msg.set_fv(fix44::CL_ORD_ID, "13346");
    msg.set_fv(
        fix44::HANDL_INST,
        fix44::HandlInst::AutomatedExecutionOrderPrivateNoBrokerIntervention,
    );
    msg.set_fv(fix44::ORD_TYPE, fix44::OrdType::Limit);
    msg.set_fv(fix44::PRICE, dec!(150.08));
    msg.set_fv(fix44::PRICE_DELTA, d128!(32.99));
    msg.set_fv(fix44::SIDE, fix44::Side::Buy);
    msg.set_fv(fix44::TIME_IN_FORCE, fix44::TimeInForce::Day);
}

fn fix_encoder() -> Encoder<Config> {
    Encoder::default()
}
