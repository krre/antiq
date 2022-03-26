use super::Window;
use std::cell::RefCell;
use std::collections::HashMap;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

thread_local! {
    static WINDOWS: RefCell<HashMap<winit::window::WindowId, Box<Window>>> = RefCell::new(HashMap::new());
}

#[derive(Debug)]
pub struct Application {
    event_loop: EventLoop<()>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
        }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::RedrawRequested(window_id) => {
                    WINDOWS.with(|w| {
                        if let Some(window) = w.borrow_mut().get(&window_id) {
                            window.render();
                        }
                    });
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    WINDOWS.with(|w| {
                        w.borrow_mut().remove(&window_id);

                        if w.borrow().len() == 0 {
                            *control_flow = ControlFlow::Exit;
                        }
                    });
                }
                _ => (),
            }
        });
    }

    pub fn name() -> Option<String> {
        std::env::current_exe()
            .ok()?
            .file_name()?
            .to_str()?
            .to_owned()
            .into()
    }

    pub fn add_window(&mut self, window: Window) {
        let box_window = Box::new(window);
        WINDOWS.with(|w| w.borrow_mut().insert(box_window.id(), box_window));
    }

    pub(crate) fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
