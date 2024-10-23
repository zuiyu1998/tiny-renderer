use parking_lot::Mutex;
use std::sync::Arc;
use tiny_renderer_macros::{Deref, DerefMut};
use wgpu::{Adapter, Buffer as WgpuBuffer, Device, Instance, Queue, RequestAdapterOptions};

use crate::renderer::resource::BufferDescriptor;

#[derive(Deref, DerefMut)]
pub struct WgpuWrapper<T>(T);

impl<T> WgpuWrapper<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

#[derive(Clone)]
pub struct RenderBackend {
    pub device: RenderDevice,
    pub queue: RenderQueue,
    pub instance: RenderInstance,
    pub adapter: RenderAdapter,
}

#[derive(Deref, DerefMut)]
pub struct RenderBuffer(WgpuWrapper<WgpuBuffer>);

impl RenderBuffer {
    pub fn new(buffer: WgpuBuffer) -> Self {
        RenderBuffer(WgpuWrapper(buffer))
    }
}

#[derive(Deref, DerefMut, Clone)]
pub struct RenderDevice(Arc<WgpuWrapper<Device>>);

impl RenderDevice {
    pub fn create_render_buffer(&self, descriptor: &BufferDescriptor) -> RenderBuffer {
        let buffer = self.create_buffer(&descriptor.get_wgpu_descriptor());

        RenderBuffer::new(buffer)
    }

    pub fn configure_surface(&self, surface: &wgpu::Surface, config: &wgpu::SurfaceConfiguration) {
        surface.configure(&self.0, config);
    }
}

#[derive(Deref, DerefMut, Clone)]
pub struct RenderQueue(Arc<WgpuWrapper<Queue>>);

#[derive(Deref, DerefMut, Clone)]
pub struct RenderInstance(Arc<WgpuWrapper<Instance>>);

#[derive(Deref, DerefMut, Clone)]
pub struct RenderAdapter(Arc<WgpuWrapper<Adapter>>);

impl RenderBackend {
    pub fn create_render_backend() -> Self {
        let future_renderer_resources_wrapper = Arc::new(Mutex::new(None));

        let future_renderer_resources_wrapper_clone = future_renderer_resources_wrapper.clone();
        let async_renderer = async move {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                ..Default::default()
            });

            let request_adapter_options = wgpu::RequestAdapterOptions {
                ..Default::default()
            };

            let (device, queue, render_adapter) =
                initialize_renderer(&instance, &request_adapter_options).await;

            let mut future_renderer_resources_inner =
                future_renderer_resources_wrapper_clone.lock();

            *future_renderer_resources_inner = Some((
                device,
                queue,
                render_adapter,
                RenderInstance(Arc::new(WgpuWrapper::new(instance))),
            ));
        };

        futures_lite::future::block_on(async_renderer);

        let (device, queue, adapter, instance) =
            future_renderer_resources_wrapper.lock().take().unwrap();

        RenderBackend {
            device,
            queue,
            adapter,
            instance,
        }
    }
}

pub async fn initialize_renderer(
    instance: &Instance,
    request_adapter_options: &RequestAdapterOptions<'_, '_>,
) -> (RenderDevice, RenderQueue, RenderAdapter) {
    let adapter = instance
        .request_adapter(request_adapter_options)
        .await
        .expect("request adapter fail");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();
    let queue = Arc::new(WgpuWrapper::new(queue));
    let adapter = Arc::new(WgpuWrapper::new(adapter));
    let device = Arc::new(WgpuWrapper::new(device));

    (
        RenderDevice(device),
        RenderQueue(queue),
        RenderAdapter(adapter),
    )
}
