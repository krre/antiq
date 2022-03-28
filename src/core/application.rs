use super::Window;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Weak;
use winit::window::WindowId;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

thread_local! {
    static EVENT_LOOP: Cell<Option<EventLoop<()>>> = Cell::new(Some(EventLoop::new()));
    static WINDOWS: RefCell<HashMap<WindowId, Weak<Window>>> = RefCell::new(HashMap::new());
}

#[derive(Debug)]
pub struct Application {}

impl Application {
    pub fn run() {
        Application::event_loop().run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::RedrawRequested(window_id) => {
                    WINDOWS.with(|w| {
                        if let Some(window) = w.borrow().get(&window_id) {
                            window.upgrade().unwrap().render();
                        }
                    });
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    WINDOWS.with(|w| {
                        let mut windows = w.borrow_mut();

                        if let Some(window) = windows.get(&window_id) {
                            window.upgrade().unwrap().set_visible(false);
                            windows.remove(&window_id);
                        }

                        if windows.len() == 0 {
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

    pub(crate) fn add_window(window: Weak<Window>) {
        let id = window.upgrade().unwrap().id();
        WINDOWS.with(|w| w.borrow_mut().insert(id, window));
    }

    pub(crate) fn event_loop() -> EventLoop<()> {
        EVENT_LOOP.with(|e| e.take().unwrap_or_else(EventLoop::new))
    }
}
