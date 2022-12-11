use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fefix::prelude::*;
use fefix::tagvalue::Decoder;
use fefix::GetConfig;

const FIX_MESSAGE: &[u8] = b"8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072|";

fn decode_fix_message(fix_decoder: &mut Decoder, msg: &[u8]) {
    fix_decoder.decode(msg).expect("Invalid FIX message");
}

fn criterion_benchmark(c: &mut Criterion) {
    let fix_dictionary = Dictionary::fix44();
    let fix_decoder = &mut Decoder::new(fix_dictionary);
    fix_decoder.config_mut().separator = b'|';
    fix_decoder.config_mut().should_decode_associative = true;
    c.bench_function("FIX tag-value decoding", |b| {
        b.iter(|| decode_fix_message(black_box(fix_decoder), black_box(FIX_MESSAGE)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
