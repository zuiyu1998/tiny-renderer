use std::error::Error;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    let mut app_state = AppState::new();

    event_loop.run_app(&mut app_state)?;

    Ok(())
}

pub struct RenderBackend {}

pub enum GraphsContext {
    Uninitialized,
    Initialized(InitializedGraphsContext),
}

pub struct InitializedGraphsContext {
    window: Window,
}

impl InitializedGraphsContext {
    pub fn new(window: Window) -> Self {
        Self { window }
    }
}

impl GraphsContext {
    pub fn initialize(&mut self, window: Window) {
        if let GraphsContext::Uninitialized = self {
            *self = GraphsContext::Initialized(InitializedGraphsContext::new(window));
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppLifecycle {
    /// 未初始化
    Idle,
    /// The application is running.
    Running,
    /// The application is going to be suspended.
    /// Applications have one frame to react to this event before being paused in the background.
    WillSuspend,
    /// The application was suspended.
    Suspended,
    /// The application is going to be resumed.
    /// Applications have one extra frame to react to this event before being fully resumed.
    WillResume,
}

pub struct AppState {
    graphs_context: GraphsContext,
    pub initialized: bool,
    lifecycle: AppLifecycle,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            graphs_context: GraphsContext::Uninitialized,
            initialized: false,
            lifecycle: AppLifecycle::Idle,
        }
    }

    pub fn initialize(&mut self, event_loop: &ActiveEventLoop) {
        self.initialized = true;

        let window = event_loop
            .create_window(WindowAttributes::default())
            .expect("create window fail.");

        self.graphs_context.initialize(window);
    }
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        self.lifecycle = AppLifecycle::WillResume;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.lifecycle == AppLifecycle::WillResume {
            if !self.initialized {
                self.initialize(event_loop);
            }
        }
    }
}
