use codegen::Scope;

struct Field {
    tag_number: usize,
    field_encoding_operator: char,
    data_type_descriptor: char,
    details: String,
}

impl Field {
    fn from_string(s: String) -> Field {
        let mut tag_number_string = String::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                tag_number_string.push(c);
            } else {
                break;
            }
        }
        Field {
            tag_number: tag_number_string.parse().unwrap(),
            field_encoding_operator: chars.next().unwrap(),
            data_type_descriptor: chars.next().unwrap(),
            details: String::new(),
        }
    }
}

struct Description {
    template_name: String,
    elements: Vec<Element>,
}

impl Description {
    fn codegen(self) -> String {
        let mut scope = Scope::new();

        let mut struct_definition = scope
            .new_struct(self.template_name.as_str())
            .derive("Debug")
            .derive("Clone");
        for element in self.elements {
            struct_definition = struct_definition.field("one", "usize");
        }
        scope.to_string()
    }
}

enum Element {
    Field(Field),
    RepeatingGroup(Vec<Element>),
}
