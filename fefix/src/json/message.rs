use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MessageBuilder {
    std_header: HashMap<String, String>,
    body: HashMap<String, String>,
    std_trailer: HashMap<String, String>,
}

impl MessageBuilder {
    pub fn empty() -> Self {
        Self {
            std_header: HashMap::new(),
            body: HashMap::new(),
            std_trailer: HashMap::new(),
        }
    }

    pub fn add_to_std_header(&mut self, tag: String, value: String) {
        self.std_header.insert(tag, value);
    }

    pub fn add_to_body(&mut self, tag: String, value: String) {
        self.body.insert(tag, value);
    }

    pub fn add_to_std_trailer(&mut self, tag: String, value: String) {
        self.std_trailer.insert(tag, value);
    }

    pub fn build(&self) -> Message {
        Message { builder: self }
    }
}

#[derive(Debug, Clone)]
pub struct Message<'a> {
    builder: &'a MessageBuilder,
}
