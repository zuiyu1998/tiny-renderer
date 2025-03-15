use std::{hash::Hash, marker::PhantomData};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RawTypeHandle {
    index: usize,
}

//类型索引
pub struct TypeHandle<T> {
    index: usize,
    _marker: PhantomData<T>,
}

impl<T> PartialEq for TypeHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self._marker == other._marker
    }
}

impl<T> Clone for TypeHandle<T> {
    fn clone(&self) -> Self {
        TypeHandle {
            index: self.index(),
            _marker: PhantomData,
        }
    }
}

impl<T> From<RawTypeHandle> for TypeHandle<T> {
    fn from(value: RawTypeHandle) -> Self {
        TypeHandle {
            index: value.index,
            _marker: PhantomData,
        }
    }
}

impl<T> TypeHandle<T> {
    pub fn raw_handle(&self) -> RawTypeHandle {
        RawTypeHandle { index: self.index }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn new(index: usize) -> Self {
        TypeHandle {
            index,
            _marker: PhantomData,
        }
    }
}
