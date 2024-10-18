use std::error::Error;

use tiny_renderer::GraphsContext;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{WindowAttributes, WindowId},
};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    let mut app_state = AppState::new();

    event_loop.run_app(&mut app_state)?;

    Ok(())
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

    //判断是否要update
    ran_update_since_last_redraw: bool,
    redraw_requested: bool,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            graphs_context: GraphsContext::Uninitialized,
            initialized: false,
            lifecycle: AppLifecycle::Idle,
            redraw_requested: false,
            ran_update_since_last_redraw: false,
        }
    }

    pub fn run_app_update(&mut self) {
        self.graphs_context.update();
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
            WindowEvent::RedrawRequested => {
                self.ran_update_since_last_redraw = false;
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let mut should_update = true;

        if self.lifecycle == AppLifecycle::WillResume {
            if !self.initialized {
                self.initialize(event_loop);
            }

            self.lifecycle = AppLifecycle::Running;
            should_update = true;
            self.redraw_requested = true;
        }

        if should_update {
            if !self.ran_update_since_last_redraw {
                self.run_app_update();

                self.ran_update_since_last_redraw = true;
            } else {
                self.redraw_requested = true;
            }
        }
    }
}
