use std::{collections::HashMap, sync::Arc};

use fyrox_core::task::TaskPool;
use fyrox_resource::manager::ResourceManager;
use tiny_renderer::{
    gfx_base::{TextureViewInfo, device::Device, texture_view::TextureView},
    gfx_wgpu::{WgpuDevice, WgpuTextureView},
    graphic_context::{GraphicContext, GraphicContextParams},
    world_renderer::{RenderCamera, RenderTarget},
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct Windows {
    primary: WindowId,
    windows: HashMap<WindowId, WindowData>,
}

pub enum CameraTarget {
    Window(Option<WindowId>),
}

pub struct Camera {
    target: CameraTarget,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            target: CameraTarget::Window(None),
        }
    }
}

impl Windows {
    pub fn get_render_camera(&self, camera: &Camera) -> Option<RenderCamera> {
        if let Some(window_data) = match &camera.target {
            CameraTarget::Window(window_id) => self.get_window(*window_id),
        } {
            let texture_view = window_data.swap_chain_texture_view.clone().unwrap();

            return Some(RenderCamera {
                render_target: RenderTarget::Window(Arc::new(texture_view)),
            });
        }

        None
    }

    pub fn get_window(&self, window_id: Option<WindowId>) -> Option<&WindowData> {
        if let Some(window_id) = window_id {
            self.windows.get(&window_id)
        } else {
            self.windows.get(&self.primary)
        }
    }

    pub fn get_primary_window(&self) -> &WindowData {
        self.windows.get(&self.primary).unwrap()
    }

    pub fn new(data: WindowData) -> Self {
        let primary = data.window.id();
        let mut windows = HashMap::default();
        windows.insert(primary, data);

        Windows { primary, windows }
    }

    pub fn add_window_data(&mut self, data: WindowData) {
        self.windows.insert(data.window.id(), data);
    }

    pub fn request_redraw(&self) {
        for window_data in self.windows.values() {
            window_data.window.request_redraw();
        }
    }

    pub fn set_swapchain_texture(&mut self) {
        for window_data in self.windows.values_mut() {
            window_data.set_swapchain_texture();
        }
    }

    pub fn present(&mut self) {
        for window_data in self.windows.values_mut() {
            window_data.present();
        }
    }
}

pub struct WindowData {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,

    pub swap_chain_texture_view: Option<TextureView>,
    pub swap_chain_texture: Option<wgpu::SurfaceTexture>,
    pub swap_chain_texture_format: Option<wgpu::TextureFormat>,
}

impl WindowData {
    pub fn new(window: Arc<Window>, surface: wgpu::Surface<'static>) -> Self {
        Self {
            window,
            surface,
            swap_chain_texture: None,
            swap_chain_texture_format: None,
            swap_chain_texture_view: None,
        }
    }

    pub fn set_swapchain_texture(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();

        let texture_view_descriptor = wgpu::TextureViewDescriptor {
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        };
        let texture_view = frame.texture.create_view(&texture_view_descriptor);

        self.swap_chain_texture_view = Some(TextureView::new(
            WgpuTextureView(texture_view),
            TextureViewInfo {},
        ));

        self.swap_chain_texture = Some(frame);
    }

    pub fn present(&mut self) {
        self.swap_chain_texture_view = None;
        self.swap_chain_texture_format = None;

        if let Some(frame) = self.swap_chain_texture.take() {
            frame.present();
        }
    }
}

struct State {
    windows: Windows,
    // device: wgpu::Device,
    // queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    graphic_context: GraphicContext,
    _resource_manager: ResourceManager,
    camera: Camera,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        surface.configure(&device, &surface_config);

        let windows = Windows::new(WindowData::new(window, surface));

        let device = WgpuDevice::new(device, queue);
        let device = Arc::new(Device::new(device));

        let task_pool = Arc::new(TaskPool::new());
        let resource_manager = ResourceManager::new(task_pool);

        let mut graphic_context = GraphicContext::Uninitialization(GraphicContextParams {});

        let (shader_event_sender, shader_event_receiver) = std::sync::mpsc::channel();

        resource_manager
            .state()
            .event_broadcaster
            .add(shader_event_sender);

        graphic_context.initialization(device, shader_event_receiver);

        State {
            windows,
            size,
            graphic_context,
            _resource_manager: resource_manager,
            camera: Camera::default(),
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
    }

    fn render(&mut self) {
        self.windows.set_swapchain_texture();

        let render_camera = vec![self.windows.get_render_camera(&self.camera).unwrap()];

        self.graphic_context.render(0.0, &render_camera);

        self.windows.present();
    }
}

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = futures_lite::future::block_on(State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
                // Emits a new redraw requested event.
                state.windows.request_redraw();
            }
            WindowEvent::Resized(size) => {
                state.resize(size);
            }
            _ => (),
        }
    }
}

fn main() {
    tracing_subscriber::fmt().init();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
