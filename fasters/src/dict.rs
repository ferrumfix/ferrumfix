use crate::repo::types;
use crate::repo::RepoV2010;
use crate::Version;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Dictionary {
    messages: HashMap<String, types::Message>,
    fields: HashMap<usize, types::Field>,
}

impl Dictionary {
    fn new(version: Version) -> Self {
        let raw_fields = RepoV2010::fields(version);
        let raw_messages = RepoV2010::messages(version);
        let mut fields = HashMap::new();
        let mut messages = HashMap::new();
        for f in raw_fields.fields.into_iter() {
            fields.insert(f.tag, f);
        }
        for m in raw_messages.data.into_iter() {
            messages.insert(m.category_id.clone(), m);
        }
        Self { messages, fields }
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "repo_v2010")]
    #[test]
    fn fix40_fields_has_at_least_100_fields() {
        use super::*;
        let dict = Dictionary::new(Version::Fix40);
        assert!(dict.fields.len() >= 100);
    }
}
