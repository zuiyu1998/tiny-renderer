use crate::frame_graph::Resource;

use super::{handle::TypeHandle, texture_view::TextureView};

#[derive(Debug)]
pub enum ColorAttachmentView {
    Uninitialization(TypeHandle<Resource>),
    Initialization(TextureView),
}

impl ColorAttachmentView {
    pub fn new(handle: TypeHandle<Resource>) -> Self {
        ColorAttachmentView::Uninitialization(handle)
    }

    pub fn get_texture_view(&self) -> &TextureView {
        match self {
            ColorAttachmentView::Uninitialization(_) => {
                unimplemented!()
            }
            ColorAttachmentView::Initialization(view) => view,
        }
    }
}

#[derive(Debug)]
pub struct ColorAttachment {
    pub view: ColorAttachmentView,
}
