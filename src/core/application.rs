use super::Window;
use std::cell::RefCell;
use std::collections::HashMap;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

thread_local! {
    static EVENT_LOOP: RefCell<Option<EventLoop<()>>> = RefCell::new(Some(EventLoop::new()));
    static WINDOWS: RefCell<HashMap<winit::window::WindowId, Box<Window>>> = RefCell::new(HashMap::new());
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

    pub fn add_window(window: Window) {
        let box_window = Box::new(window);
        WINDOWS.with(|w| w.borrow_mut().insert(box_window.id(), box_window));
    }

    pub(crate) fn event_loop() -> EventLoop<()> {
        EVENT_LOOP.with(|el| el.borrow_mut().take().unwrap_or_else(EventLoop::new))
    }
}
