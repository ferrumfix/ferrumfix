use crate::Dictionary;

#[derive(Debug, Clone)]
pub struct TagMap<T> {
    data: Vec<(T, u32)>,
}

impl<T> TagMap<T> {
    pub fn new<F>(_dict: Dictionary, _filler: F) -> Self
    where
        F: Fn(u32) -> T,
    {
        Self { data: Vec::new() }
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
