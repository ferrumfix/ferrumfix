use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fefix::tagvalue::{Config, Decoder};
use fefix::{AppVersion, Dictionary};

const FIX_MESSAGE: &[u8] = b"8=FIX.4.2|9=97|35=6|49=BKR|56=IM|34=14|52=20100204-09:18:42|23=115685|28=N|55=SPMI.MI|54=2|44=2200.75|27=S|25=H|10=248|";

fn decode_fix_message(fix_decoder: &mut Decoder<Config>, msg: &[u8]) {
    fix_decoder.decode(msg).expect("Invalid FIX message");
}

fn criterion_benchmark(c: &mut Criterion) {
    let fix_dictionary = Dictionary::from_version(AppVersion::Fix42);
    let fix_decoder = &mut Decoder::<Config>::new(fix_dictionary);
    fix_decoder.config_mut().set_separator(b'|');
    fix_decoder.config_mut().set_verify_checksum(false);
    fix_decoder.config_mut().set_decode_assoc(true);
    fix_decoder.config_mut().set_decode_seq(false);
    c.bench_function("FIX tag-value decoding", |b| {
        b.iter(|| decode_fix_message(black_box(fix_decoder), black_box(FIX_MESSAGE)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
