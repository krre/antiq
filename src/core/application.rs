use super::Window;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

thread_local! {
    static EVENT_LOOP: Cell<Option<EventLoop<()>>> = Cell::new(Some(EventLoop::new()));
    static WINDOWS: RefCell<HashMap<winit::window::WindowId, Rc<RefCell<Window>>>> = RefCell::new(HashMap::new());
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
                            window.borrow().render();
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

    pub(crate) fn add_window(window: Window) -> Weak<RefCell<Window>> {
        let rc_window = Rc::new(RefCell::new(window));
        let weak_window = Rc::downgrade(&rc_window);
        let id = rc_window.borrow().id();
        WINDOWS.with(|w| w.borrow_mut().insert(id, rc_window));
        weak_window
    }

    pub(crate) fn event_loop() -> EventLoop<()> {
        EVENT_LOOP.with(|e| e.take().unwrap_or_else(EventLoop::new))
    }
}
