use std::{fmt::Debug, hash::Hash, marker::PhantomData};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RawTypeHandle {
    index: usize,
}

//类型索引
pub struct TypeHandle<T> {
    index: usize,
    _marker: PhantomData<T>,
}

impl<T> Debug for TypeHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeHandle")
            .field("index", &self.index)
            .finish()
    }
}

impl<T> PartialEq for TypeHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self._marker == other._marker
    }
}

impl<T> Copy for TypeHandle<T> {}

impl<T> Clone for TypeHandle<T> {
    fn clone(&self) -> Self {
        *self
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
