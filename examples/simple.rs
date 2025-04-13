use std::{collections::HashMap, sync::Arc};

use fyrox_core::task::TaskPool;
use fyrox_resource::manager::ResourceManager;
use tiny_renderer::{
    gfx_base::{TextureViewInfo, device::Device, texture_view::TextureView},
    gfx_wgpu::{WgpuDevice, WgpuTextureView},
    graphic_context::{GraphicContext, GraphicContextParams},
    world_renderer::{CameraInfo, RenderCamera, RenderTarget},
};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, KeyCode, NamedKey, PhysicalKey},
    window::{Window, WindowId},
};

struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
        }
    }

    fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        logical_key,
                        physical_key,
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                if let Key::Named(NamedKey::Space) = logical_key {
                    self.is_up_pressed = is_pressed;
                    return true;
                }
                match physical_key {
                    PhysicalKey::Code(KeyCode::ShiftLeft) => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.info.target - camera.info.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        // 防止摄像机离场景中心太近时出现问题
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.info.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.info.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.info.up);

        // 在按下前进或后退键时重做半径计算
        let forward = camera.info.target - camera.info.eye;
        let forward_mag = forward.length();

        if self.is_right_pressed {
            // 重新调整目标和眼睛之间的距离，以便其不发生变化。
            // 因此，眼睛仍然位于目标和眼睛形成的圆圈上。
            camera.info.eye =
                camera.info.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.info.eye =
                camera.info.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}

pub struct Windows {
    primary: WindowId,
    windows: HashMap<WindowId, WindowData>,
}

pub enum CameraTarget {
    Window(Option<WindowId>),
}

impl Default for CameraTarget {
    fn default() -> Self {
        CameraTarget::Window(None)
    }
}

#[derive(Default)]
pub struct Camera {
    target: CameraTarget,
    info: CameraInfo,
}

impl Camera {
    pub fn set_camera_info(&mut self, width: f32, height: f32) {
        self.info.aspect = width / height;
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
                info: camera.info.clone(),
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
    camera_controller: CameraController,
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

        let camera = Camera {
            info: CameraInfo {
                // 将摄像机向上移动 1 个单位，向后移动 2 个单位
                // +z 朝向屏幕外
                eye: (0.0, 1.0, 2.0).into(),
                // 摄像机看向原点
                target: (0.0, 0.0, 0.0).into(),
                // 定义哪个方向朝上
                up: glam::Vec3::Y,
                aspect: size.width as f32 / size.height as f32,
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
            },
            ..Default::default()
        };

        State {
            windows,
            size,
            graphic_context,
            _resource_manager: resource_manager,
            camera,
            camera_controller: CameraController::new(10.0),
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
        state.camera_controller.process_events(&event);
        state.camera_controller.update_camera(&mut state.camera);

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
