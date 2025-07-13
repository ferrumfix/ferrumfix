use rustyfix::json::FieldOrGroup;
use rustyfix::prelude::*;

const JSON_FIX_MESSAGE: &str = include_str!("fix-example.json");

fn main() {
    let dictionary = rustyfix::Dictionary::fix42().expect("Failed to load FIX 4.2 dictionary");
    let mut decoder = rustyfix::json::Decoder::new(dictionary.clone());
    let mut encoder = rustyfix::tagvalue::Encoder::new();
    let mut buffer = Vec::new();

    let mut json_bytes = JSON_FIX_MESSAGE.as_bytes().to_vec();
    let json_msg = decoder
        .decode(&mut json_bytes)
        .expect("Failed to decode JSON FIX message");
    let msg_type: &str = json_msg
        .get(fix42::MSG_TYPE)
        .expect("MSG_TYPE field not found in FIX message");
    let begin_string: &str = json_msg
        .get(fix42::BEGIN_STRING)
        .expect("BEGIN_STRING field not found in FIX message");

    let mut fix_msg_builder =
        encoder.start_message(begin_string.as_bytes(), &mut buffer, msg_type.as_bytes());

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
mod test {
    use super::*;

    #[test]
    fn run() {
        main();
    }
}
