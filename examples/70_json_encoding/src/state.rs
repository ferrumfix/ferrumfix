use std::collections::HashMap;

use deadpool::unmanaged::{Object, Pool};
use fefix::{json, tagvalue, Dictionary, TagU16};

const POOL_CAPACITY: usize = 16;

pub type Decoder = json::Decoder<json::Config>;
pub type Encoder = tagvalue::Encoder<Vec<u8>, tagvalue::Config>;

#[derive(Clone)]
pub struct State {
    field_registry: HashMap<String, TagU16>,
    pool: Pool<(Decoder, Encoder)>,
}

impl State {
    pub fn new() -> Self {
        let fix_dictionary = Dictionary::fix42();
        let pool = Pool::new(POOL_CAPACITY);
        for _ in 0..POOL_CAPACITY {
            let decoder = Decoder::new(fix_dictionary.clone());
            let encoder = Encoder::from_buffer(Vec::new());
            pool.try_add((decoder, encoder)).ok();
        }
        let mut field_registry = HashMap::new();
        for field in fix_dictionary.iter_fields() {
            field_registry.insert(field.name().to_string(), field.tag());
        }
        Self {
            field_registry,
            pool,
        }
    }

    pub async fn codec(&self) -> Object<(Decoder, Encoder)> {
        self.pool.get().await
    }

    pub fn lookup_field_name(&self, field: &str) -> Option<TagU16> {
        self.field_registry.get(field).copied()
    }
}
