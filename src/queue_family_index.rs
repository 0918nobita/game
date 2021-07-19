use std::marker::PhantomData;

pub struct Graphics;

pub struct QueueFamilyIndex<T> {
    pub raw_index: u32,
    _phantom: PhantomData<T>,
}

impl<T> QueueFamilyIndex<T> {
    pub fn new(raw_index: u32) -> Self {
        QueueFamilyIndex {
            raw_index,
            _phantom: PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for QueueFamilyIndex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.raw_index)
    }
}
