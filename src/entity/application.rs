use crate::core::EntityCore;
use crate::entity::Entity;
use crate::widget::WindowWidget;
use std::cell::RefCell;
use std::collections::HashMap;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

thread_local! {
    static WINDOWS: RefCell<HashMap<winit::window::WindowId, Box<dyn WindowWidget>>> = RefCell::new(HashMap::new());
}

#[derive(Debug)]
pub struct Application {
    entity_core: EntityCore,
    event_loop: EventLoop<()>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            entity_core: EntityCore::default(),
            event_loop: EventLoop::new(),
        }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            println!("{:?}", event);
            match event {
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

        Ok(())
    }

    pub fn name() -> Option<String> {
        std::env::current_exe()
            .ok()?
            .file_name()?
            .to_str()?
            .to_owned()
            .into()
    }

    pub fn add_window<W: WindowWidget + 'static>(&mut self, window: W) {
        let box_window = Box::new(window);
        WINDOWS.with(|w| w.borrow_mut().insert(box_window.id(), box_window));
    }

    pub(crate) fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }
}

impl Entity for Application {
    fn entity_ref(&self) -> &EntityCore {
        &self.entity_core
    }

    fn entity_mut(&mut self) -> &mut EntityCore {
        &mut self.entity_core
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
