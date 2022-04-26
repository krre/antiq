use super::window;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use winit::window::WindowId;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

thread_local! {
    static EVENT_LOOP: Cell<Option<EventLoop<()>>> = Cell::new(Some(EventLoop::new()));
    static WINDOWS: RefCell<HashMap<WindowId, Weak<window::Window>>> = RefCell::new(HashMap::new());
}

#[derive(Debug)]
pub struct Application {}

impl Application {
    pub fn run() {
        Application::event_loop().run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::RedrawRequested(window_id) => {
                    WINDOWS.with(|ws| {
                        if let Some(window) = ws.borrow().get(&window_id) {
                            window.upgrade().unwrap().draw();
                        }
                    });
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    WINDOWS.with(|ws| {
                        let mut window: Option<Rc<window::Window>> = None;

                        {
                            let windows = ws.borrow();

                            if let Some(w) = windows.get(&window_id) {
                                window = w.upgrade();
                            }
                        }

                        if let Some(w) = window {
                            w.set_visible(false);
                        }

                        if ws.borrow().len() == 0 {
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

    pub(crate) fn add_window(window: Weak<window::Window>) {
        let id = window.upgrade().unwrap();
        WINDOWS.with(|ws| ws.borrow_mut().insert(id.winit_id(), window));
    }

    pub(crate) fn remove_window(id: &window::Id) {
        WINDOWS.with(|ws| ws.borrow_mut().remove(&id.winit_id()));
    }

    pub(crate) fn event_loop() -> EventLoop<()> {
        EVENT_LOOP.with(|e| e.take().unwrap_or_else(EventLoop::new))
    }
}
