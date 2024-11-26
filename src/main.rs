#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::all)]
#![warn(clippy::nursery)]

use anyhow::Result;
use pollster::FutureExt;
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::*;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowAttributes, WindowId};

fn main() -> Result<()> {
    App::run()
}

struct App<'a> {
    state: Option<State<'a>>,
}

impl<'a> App<'a> {
    pub fn run() -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut App::init())?;
        Ok(())
    }

    const fn init() -> Self {
        Self { state: None }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.state = Some(State::init(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();

        // let window = match state.windows.get_mut(&window_id) {
        //     Some(window) => window,
        //     None => return,
        // };

        match event {
            WindowEvent::CloseRequested => {
                state.windows.remove(&window_id);
            }
            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {
                if event.state.is_pressed()
                    && event.logical_key == Key::Named(NamedKey::Alt)
                    && event.logical_key == Key::Named(NamedKey::F4)
                {
                    event_loop.exit();
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let state = self.state.as_ref().unwrap();

        if state.windows.is_empty() {
            event_loop.exit();
        }
    }
}

struct State<'s> {
    instance: Instance,
    device: Device,
    queue: Queue,
    windows: HashMap<WindowId, WindowState<'s>>,
}

impl<'s> State<'s> {
    fn init(event_loop: &ActiveEventLoop) -> Self {
        let instance = Instance::default();

        let (device, queue) = async {
            let adapter = instance
                .request_adapter(&RequestAdapterOptions::default())
                .await
                .unwrap();

            adapter
                .request_device(&DeviceDescriptor::default(), None)
                .await
                .unwrap()
        }
        .block_on();

        let window_attributes = WindowAttributes::default().with_title(env!("CARGO_PKG_NAME"));
        let main_window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let surface = instance.create_surface(main_window.clone()).unwrap();

        let window_state = WindowState {
            surface,
            window: main_window,
        };

        let windows = HashMap::from([(window_state.window.id(), window_state)]);

        let mut state = Self {
            instance,
            device,
            queue,
            windows,
        };

        state.init_equi();
        state
    }

    fn init_equi(&mut self) {}
}

struct WindowState<'w> {
    surface: Surface<'w>,
    window: Arc<Window>,
}
