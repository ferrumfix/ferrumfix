use crate::Dictionary;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TagMap<T> {
    assocative_table: Vec<(u32, T)>,
    fallback: HashMap<u32, T>,
}

impl<T> TagMap<T> {
    pub fn new<F>(_dict: Dictionary, _filler: F) -> Self
    where
        F: Fn(u32) -> T,
    {
        Self {
            assocative_table: Vec::new(),
            fallback: HashMap::default(),
        }
    }

    pub fn get(&self) -> &T {
        unimplemented!()
    }

    pub fn get_opt(&self) -> Option<&T> {
        None
    }

    pub fn get_mut(&mut self) -> &mut T {
        unimplemented!()
    }

    pub fn get_mut_opt(&mut self) -> Option<&mut T> {
        unimplemented!()
    }
}
