use fefix::json::FieldOrGroup;
use fefix::prelude::*;

const JSON_FIX_MESSAGE: &str = include_str!("fix-example.json");

fn main() {
    let dictionary = fefix::Dictionary::fix42();
    let mut decoder = fefix::json::Decoder::new(dictionary.clone());
    let mut encoder = fefix::tagvalue::Encoder::new();
    let mut buffer = Vec::new();

    let json_msg = decoder.decode(JSON_FIX_MESSAGE.as_bytes()).unwrap();
    let msg_type = json_msg.fv(fix42::MSG_TYPE).unwrap();
    let begin_string = json_msg.fv(fix42::BEGIN_STRING).unwrap();

    let mut fix_msg_builder = encoder.start_message(begin_string, &mut buffer, msg_type);

    for (field_name, field_value) in json_msg.iter_fields() {
        let field = dictionary
            .field_by_name(field_name)
            .expect("Invalid FIX.4.2 field!");

        match field_value {
            FieldOrGroup::Field(s) => {
                fix_msg_builder.set(field.tag(), s.as_ref());
            }
            FieldOrGroup::Group(_g) => {}
        }
    }

    let fix_msg = fix_msg_builder.done().0;

    println!("Successful conversion from JSON syntax to tag=value|.");
    println!();
    println!("{}", String::from_utf8_lossy(fix_msg));
}

#[cfg(test)]
#[test]
fn run() {
    main();
}
