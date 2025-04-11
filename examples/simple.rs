use std::sync::Arc;

use fyrox_core::task::TaskPool;
use fyrox_resource::manager::ResourceManager;
use tiny_renderer::{
    gfx_base::device::Device,
    gfx_wgpu::WgpuDevice,
    graphic_context::{GraphicContext, GraphicContextParams},
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

struct State {
    window: Arc<Window>,
    // device: wgpu::Device,
    // queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    graphic_context: GraphicContext,
    _resource_manager: ResourceManager,
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

        let device = WgpuDevice::new(device, surface, surface_format, queue);
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
            window,
            size,
            graphic_context,
            _resource_manager: resource_manager,
        }
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
    }

    fn render(&mut self) {
        self.graphic_context.render(0.0);

        // Renders a GREEN screen
        // let mut encoder = self.device.create_command_encoder(&Default::default());
        // // Create the renderpass which will clear the screen.
        // let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //     label: None,
        //     color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        //         view: &texture_view,
        //         resolve_target: None,
        //         ops: wgpu::Operations {
        //             load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
        //             store: wgpu::StoreOp::Store,
        //         },
        //     })],
        //     depth_stencil_attachment: None,
        //     timestamp_writes: None,
        //     occlusion_query_set: None,
        // });

        // // If you wanted to call any drawing commands, they would go here.

        // // End the renderpass.
        // drop(renderpass);

        // // Submit the command in the queue to execute
        // self.queue.submit([encoder.finish()]);
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
                state.get_window().request_redraw();
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
