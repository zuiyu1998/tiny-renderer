use downcast_rs::Downcast;
use wgpu::{BufferAddress, BufferUsages};

use crate::{define_atomic_id, define_gfx_frame_graph_type};
use std::{borrow::Cow, fmt::Debug};

define_atomic_id!(BufferId);

pub trait BufferTrait: 'static + Debug + Sync + Send {}

pub trait ErasedBufferTrait: 'static + Downcast + Debug + Sync + Send {}

impl<T: BufferTrait> ErasedBufferTrait for T {}

define_gfx_frame_graph_type!(Buffer, BufferId, BufferTrait, ErasedBufferTrait, BufferInfo);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BufferInfo {
    pub label: Option<Cow<'static, str>>,
    pub size: BufferAddress,
    pub usage: BufferUsages,
    pub mapped_at_creation: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BufferInitInfo<'a> {
    pub label: Option<Cow<'static, str>>,
    pub contents: &'a [u8],
    pub usage: BufferUsages,
}
