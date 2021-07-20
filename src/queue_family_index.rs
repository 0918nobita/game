use std::{fmt, marker::PhantomData};

pub struct Graphics;

pub struct QueueFamilyIndex<T> {
    raw_index: u32,
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

impl<T> fmt::Debug for QueueFamilyIndex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.raw_index)
    }
}

impl<T> std::ops::Deref for QueueFamilyIndex<T> {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.raw_index
    }
}
