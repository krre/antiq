use std::{
    any::Any,
    cell::{Cell, RefCell},
    rc::Rc,
    sync::atomic::Ordering,
};

use crate::{
    core::{
        Result, Size2D,
        event::{Event, EventHandler, WindowAction, WindowEvent},
    },
    platform::{PlatformApplication, PlatformEventLoop},
};

use wayland_client::{Connection, Dispatch, EventQueue, QueueHandle};
use wayland_protocols::xdg::shell::client::{
    xdg_surface::{self, XdgSurface},
    xdg_toplevel::{self, XdgToplevel},
    xdg_wm_base::{self, XdgWmBase},
};

use super::{Application, XdgSurfaceData, XdgToplevelData};

pub struct EventLoop {
    event_queue: RefCell<EventQueue<State>>,
    pub(crate) xdg_wm_base: XdgWmBase,
    pub(crate) queue_handle: QueueHandle<State>,
    running: Cell<bool>,
}

pub(crate) struct State {
    event_handler: Box<dyn EventHandler>,
}

impl EventLoop {
    pub fn new(application: Rc<dyn PlatformApplication>) -> Result<Rc<dyn PlatformEventLoop>> {
        let application = Rc::downcast::<Application>(application.clone() as Rc<dyn Any>).unwrap();
        let event_queue = RefCell::new(application.connection.new_event_queue());
        let queue_handle = event_queue.borrow().handle();
        let xdg_wm_base: XdgWmBase = application.globals.bind(&queue_handle, 5..=6, ()).unwrap();

        Ok(Rc::new(Self {
            event_queue,
            queue_handle,
            xdg_wm_base,
            running: Cell::new(false),
        }))
    }
}

impl PlatformEventLoop for EventLoop {
    fn process_events(&self, event_handler: Box<dyn EventHandler>) -> Result<()> {
        self.running.set(true);

        let mut state = State { event_handler };

        while self.running.get() {
            self.event_queue
                .borrow_mut()
                .blocking_dispatch(&mut state)?;
        }

        Ok(())
    }

    fn send_event(&self, _event: Box<dyn Event>) {}

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
            xdg_toplevel::Event::Configure {
                width,
                height,
                states: _,
            } => {
                if width != 0 && height != 0 {
                    if data.is_inited.load(Ordering::Relaxed) == true {
                        state.event_handler.window_event(WindowEvent {
                            id: data.window_id,
                            action: WindowAction::AskResize(Size2D::new(
                                width as u32,
                                height as u32,
                            )),
                        });
                    } else {
                        data.is_inited.store(true, Ordering::Relaxed);
                    }
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
