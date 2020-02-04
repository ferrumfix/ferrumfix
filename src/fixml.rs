use crate::err::{Error, Result};
use crate::template::Template;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::Reader;

pub struct State<'s> {
    template: Template,
    is_root: bool,
    reader: Reader<&'s [u8]>,
}

impl<'s> State<'s> {
    pub fn new(xml: &'s str) -> Self {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        State {
            template: Template::new(),
            is_root: true,
            reader,
        }
    }

    pub fn consume(mut self) -> Result<Template> {
        let mut buf = Vec::new();
        loop {
            match self.reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => self.open_tag(e.name(), e.attributes())?,
                Ok(Event::End(ref e)) => unimplemented!(),
                Ok(Event::Empty(ref e)) => unimplemented!(),
                Ok(Event::Eof) => break,
                Ok(_) => (),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(self.template)
    }

    fn open_tag(&mut self, name: &[u8], attrs: Attributes) -> Result<()> {
        if self.is_root && name == b"FIXML" {
            for attr in attrs {
                if attr?.key == b"v" {
                    // TODO: automatically get dictionary.
                }
            }
        } else if self.is_root {
            return Err(Error::BadFixml);
        } else {
            // TODO.
        }
        self.is_root = false;
        Ok(())
    }

    fn transition_close_tag(&mut self, name: String) -> Result<()> {
        Ok(())
    }

    fn transition_eof(&mut self) -> Result<()> {
        Ok(())
    }
}
