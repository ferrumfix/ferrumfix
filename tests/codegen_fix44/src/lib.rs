#[rustfmt::skip]
#[allow(dead_code)]
mod generated_fix44;

#[cfg(test)]
mod tests {
    use crate::generated_fix44 as fields;
    use rustyfix::dict::IsFieldDefinition;

    #[test]
    fn info_about_begin_string_field_is_correct() {
        assert_eq!(fields::BEGIN_STRING.name(), "BeginString");
        assert_eq!(fields::BEGIN_STRING.tag().get(), 8);
    }
}
