#[derive(Default)]
pub struct Tagvalue {
    data: String,
    checksum: Checksum,
}

impl Tagvalue {
    const SEPARATOR: char = 1u8 as char;

    fn write_tag(&mut self, name: &str) {
        self.data.push_str(name);
        self.data.push('=');
        // TODO: value.
        self.data.push_str("value");
        self.data.push(Self::SEPARATOR);
    }

    fn write_tag_header(&mut self) {
        unimplemented!()
    }
}

pub trait Fix {
    fn as_tagvalue(&self) -> String;
    fn as_fixml(&self) -> String;
    fn gpb(&self) -> String;
    fn sbe(&self) -> String;
    fn json(&self) -> String;
    fn asn1(&self) -> String;
    fn fast(&self) -> String;
}

pub struct Checksum {
    value: u8,
}

impl Checksum {
    fn feed(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.value = self.value.wrapping_add(*byte);
        }
    }

    fn final_value(self) -> u8 {
        self.value
    }
}

impl Default for Checksum {
    fn default() -> Self {
        Checksum { value: 0 }
    }
}
