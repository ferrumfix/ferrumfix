#[rustfmt::skip]
mod generated_fix44;

#[cfg(test)]
mod tests {
    use crate::generated_fix44 as fields;
    use fefix::DataType;
    
    #[test]
    fn info_about_begin_string_field_is_correct() {
        assert_eq!(fields::BEGIN_STRING.name(), "BeginString");
        assert_eq!(fields::BEGIN_STRING.tag(), 8);
        assert_eq!(fields::BEGIN_STRING.data_type(), DataType::String);
    }
}
