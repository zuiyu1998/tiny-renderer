use std::{borrow::Cow, fmt::Debug};

use downcast_rs::Downcast;

use crate::{
    define_atomic_id, define_gfx_type,
    frame_graph::{GpuRead, ResourceNodeRef, ResourceTable},
};

use super::{
    BindGroupLayout, Sample, SampleInfo, Texture, TextureView, TextureViewInfo, device::Device,
};

define_atomic_id!(BindGroupId);

pub trait BindGroupTrait: ErasedBindGroupTrait {}

pub trait ErasedBindGroupTrait: 'static + Downcast + Debug + Sync + Send {}

impl<T> ErasedBindGroupTrait for T where T: BindGroupTrait {}

define_gfx_type!(BindGroup, BindGroupId, BindGroupTrait, ErasedBindGroupTrait);

pub struct BindGroupRef {
    pub label: Option<Cow<'static, str>>,
    pub layout: BindGroupLayout,
    pub entries: Vec<BindGroupEntryInfo>,
    pub index: u32,
}

impl BindGroupRef {
    pub fn get_info(&self, device: &Device, resource_table: &ResourceTable) -> BindGroupInfo {
        let mut entries = vec![];

        for entry in self.entries.iter() {
            match &entry.resource {
                BindingResourceInfo::Sampler(info) => {
                    entries.push(BindGroupEntry {
                        binding: entry.binding,
                        resource: BindingResource::Sampler(device.create_sampler(info.clone())),
                    });
                }
                BindingResourceInfo::TextureView(handle) => {
                    let resource = resource_table
                        .get_resource::<Texture>(&handle.resource_handle())
                        .unwrap();

                    entries.push(BindGroupEntry {
                        binding: entry.binding,
                        resource: BindingResource::TextureView(
                            resource.get_texture_view(TextureViewInfo {}),
                        ),
                    });
                }
            }
        }

        BindGroupInfo {
            label: self.label.clone(),
            layout: self.layout.clone(),
            entries,
        }
    }
}

pub struct BindGroupInfo {
    pub label: Option<Cow<'static, str>>,
    pub layout: BindGroupLayout,
    pub entries: Vec<BindGroupEntry>,
}

pub struct BindGroupEntryInfo {
    pub binding: u32,
    pub resource: BindingResourceInfo,
}

pub enum BindingResourceInfo {
    TextureView(ResourceNodeRef<Texture, GpuRead>),
    Sampler(SampleInfo),
}

pub struct BindGroupEntry {
    pub binding: u32,
    pub resource: BindingResource,
}

pub enum BindingResource {
    TextureView(TextureView),
    Sampler(Sample),
}
