use crate::TagU16;
use fnv::FnvHashSet;
use std::sync::Arc;

#[derive(Debug)]
pub struct TagMap<T> {
    data: Arc<Vec<(u16, T)>>,
    tag_relevant_bits: usize,
}

impl<T> TagMap<T>
where
    T: PartialEq + Eq + Clone,
{
    pub fn new(data: impl Iterator<Item = (TagU16, T)>, filler: T) -> Self {
        let mut hashset = FnvHashSet::default();
        let data: Vec<(TagU16, T)> = data.collect();
        let mut tag_relevant_bits = 1;
        while tag_relevant_bits < 16 {
            hashset.clear();
            for (tag, value) in data.iter() {
                let mask = (1 << tag_relevant_bits) - 1;
                let masked_tag = tag.get() & mask;
                if hashset.insert(masked_tag) {
                    tag_relevant_bits += 1;
                    continue;
                }
            }
        }
        let mut vec = Vec::new();
        for _ in 0..(2 << tag_relevant_bits) {
            vec.push((0, filler.clone()));
        }
        for (tag, value) in data.iter() {
            let mask = (1 << tag_relevant_bits) - 1;
            let masked_tag = tag.get() & mask;
            vec[masked_tag as usize] = (tag.get(), value.clone());
        }
        Self {
            data: Arc::new(vec),
            tag_relevant_bits,
        }
    }

    #[inline]
    pub fn get(&self, tag: TagU16) -> Option<&T> {
        let i = tag.get() as usize & self.tag_mask();
        let contents = &self.data[i];
        if contents.0 == tag.get() {
            Some(&contents.1)
        } else {
            None
        }
    }

    fn tag_mask(&self) -> usize {
        (1usize << (self.tag_relevant_bits - 1)) - 1
    }
}
