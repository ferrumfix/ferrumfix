#[derive(Debug, Clone)]
pub struct MsgTypeMap<T> {
    phantom: std::marker::PhantomData<T>,
}
