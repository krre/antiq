use std::{any::Any, cell::{Cell, RefCell}, rc::Rc};

use crate::{
    core::{
        event::{Event, EventHandler, WindowAction, WindowEvent}, Result, Size2D
    },
    platform::{PlatformApplication, PlatformEventLoop},
};

use wayland_client::{Connection, Dispatch, EventQueue, QueueHandle};
use wayland_protocols::xdg::shell::client::{xdg_surface::{self, XdgSurface}, xdg_toplevel::{self, XdgToplevel}, xdg_wm_base::{self, XdgWmBase}};

use super::{Application, XdgSurfaceData, XdgToplevelData};

pub struct EventLoop {
    application: Rc<dyn PlatformApplication>,
    event_queue: RefCell<EventQueue<State>>,
    pub(crate) xdg_wm_base: XdgWmBase,
    pub(crate) queue_handle: QueueHandle<State>,
    running: Cell<bool>
}

pub(crate) struct State {
    event_handler: Box<dyn EventHandler>,
}

impl EventLoop {
    pub fn new(application: Rc<dyn PlatformApplication>) -> Result<Rc<dyn PlatformEventLoop>> {
        let wayland_app = application.as_any().downcast_ref::<Application>().unwrap();
        let wayland_conn = wayland_app.connection.as_ref();
        let event_queue = RefCell::new(wayland_conn.new_event_queue());
        let queue_handle = event_queue.borrow().handle();
        let xdg_wm_base: XdgWmBase = wayland_app.globals.bind(&queue_handle, 5..=6, ()).unwrap();

        Ok(Rc::new(Self {
            application,
            event_queue,
            queue_handle,
            xdg_wm_base,
            running: Cell::new(false),
        }))
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

    fn process_events(&self, event_handler: Box<dyn EventHandler>) -> Result<()> {
        self.running.set(true);

        let mut state = State {
            event_handler,
        };

        while self.running.get() {
            self.event_queue
                .borrow_mut()
                .blocking_dispatch(&mut state)?;
        }

        Ok(())
    }

    fn send_event(&self, event: Box<dyn Event>) {}

    fn quit(&self) {
        self.running.set(false);
    }
}

impl Dispatch<XdgSurface, XdgSurfaceData> for State {
    fn event(
        state: &mut Self,
        xdg_surface: &XdgSurface,
        event: xdg_surface::Event,
        data: &XdgSurfaceData,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial, .. } = event {
            xdg_surface.ack_configure(serial);

            state.event_handler.window_event(WindowEvent {
                id: data.window_id,
                action: WindowAction::Redraw,
            });
        }
    }
}

impl Dispatch<XdgWmBase, ()> for State {
    fn event(
        _: &mut Self,
        wm_base: &XdgWmBase,
        event: xdg_wm_base::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm_base.pong(serial);
        }
    }
}

impl Dispatch<XdgToplevel, XdgToplevelData> for State {
    fn event(
        state: &mut Self,
        _: &XdgToplevel,
        event: xdg_toplevel::Event,
        data: &XdgToplevelData,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            xdg_toplevel::Event::Configure { width, height, states: _ }  => {
                if width != 0 && height != 0 {
                    state.event_handler.window_event(WindowEvent {
                        id: data.window_id,
                        action: WindowAction::AskResize(Size2D::new(width as u32, height as u32)),
                    });
                }
            }
            xdg_toplevel::Event::Close => {
                state.event_handler.window_event(WindowEvent {
                    id: data.window_id,
                    action: WindowAction::Close,
                });
            }
            _ => {}
        }
    }
}
