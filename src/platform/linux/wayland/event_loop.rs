use std::{any::Any, cell::RefCell, rc::Rc};

use crate::{
    core::event::{Event, EventHandler},
    platform::{PlatformApplication, PlatformEventLoop},
};

use wayland_client::{delegate_noop, protocol::{wl_buffer::WlBuffer, wl_shm_pool::WlShmPool, wl_surface::WlSurface}, Connection, EventQueue, QueueHandle};
use wayland_protocols::xdg::shell::client::{xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase};

use super::Application;

pub struct EventLoop {
    application: Rc<dyn PlatformApplication>,
    event_queue: RefCell<EventQueue<State>>,
    pub(crate) queue_handle: QueueHandle<State>,
    pub(crate) xdg_wm_base: XdgWmBase
}

pub(crate) struct State {
    running: bool,
}

delegate_noop!(State: ignore WlSurface);
delegate_noop!(State: ignore WlShmPool);
delegate_noop!(State: ignore WlBuffer);
delegate_noop!(State: ignore XdgWmBase);
delegate_noop!(State: ignore XdgSurface);
delegate_noop!(State: ignore XdgToplevel);

impl EventLoop {
    pub fn new(
        application: Rc<dyn PlatformApplication>,
    ) -> Result<Rc<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        let wayland_app = application
        .as_any()
        .downcast_ref::<Application>()
        .unwrap();
        let wayland_conn = wayland_app.connection.as_ref();
        let event_queue = RefCell::new(wayland_conn.new_event_queue());
        let queue_handle = event_queue.borrow().handle();
        let xdg_wm_base: XdgWmBase = wayland_app.globals.bind(&queue_handle, 5..=6, ()).unwrap();

        Ok(Rc::new(Self { application, event_queue, queue_handle, xdg_wm_base }))
    }

    fn application(&self) -> &Application {
        self.application
            .as_any()
            .downcast_ref::<Application>()
            .unwrap()
    }

    fn conn(&self) -> &Connection {
        self.application().connection.as_ref()
    }
}

impl PlatformEventLoop for EventLoop {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn run(&self, event_handler: &dyn EventHandler) -> Result<(), Box<dyn std::error::Error>> {
        let mut state = State { running: true };

        while state.running {
            self.event_queue.borrow_mut().blocking_dispatch(&mut state)?;
        }

        Ok(())
    }

    fn send_event(&self, event: Box<dyn Event>) {}

    fn quit(&self) {}
}
