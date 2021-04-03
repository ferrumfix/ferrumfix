use crate::Serialize;

pub trait MessageAccumulator {
    fn set_field<T>(&mut self, tag: u32, value: T)
    where
        T: Serialize;

    fn set_begin_string(&mut self, value: &[u8]);

    fn wrap_std_header(&mut self);
    fn wrap_body(&mut self);
    fn wrap_std_trailer(&mut self);
}
