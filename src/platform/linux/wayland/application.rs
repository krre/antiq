use std::{any::Any, rc::Rc, sync::Arc};
use wayland_client::{
    delegate_noop, globals::{registry_queue_init, GlobalList, GlobalListContents}, protocol::{
        wl_compositor::WlCompositor,
        wl_registry::{self, WlRegistry},
        wl_shm::WlShm,
    }, Connection, Dispatch, QueueHandle
};
use wayland_protocols::xdg::decoration::zv1::client::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1;

use crate::core::Result;
use crate::platform::PlatformApplication;

pub struct Application {
    pub(crate) connection: Arc<Connection>,
    pub(crate) globals: GlobalList,
    pub(crate) compositor: WlCompositor,
    pub(crate) shm: WlShm,
    pub(crate) xdg_decoration_manager: ZxdgDecorationManagerV1,
}

struct State {}

delegate_noop!(State: ignore WlCompositor);
delegate_noop!(State: ignore WlShm);
delegate_noop!(State: ignore ZxdgDecorationManagerV1);

impl Application {
    pub fn new() -> Result<Rc<dyn PlatformApplication>> {
        let connection = Arc::new(Connection::connect_to_env()?);
        let (globals, queue) = registry_queue_init::<State>(&connection).unwrap();
        let qh = queue.handle();
        let compositor: WlCompositor = globals.bind(&qh, 4..=5, ()).unwrap();
        let shm: WlShm = globals.bind(&qh, 1..=1, ()).unwrap();
        let xdg_decoration_manager: ZxdgDecorationManagerV1 = globals.bind(&qh, 1..=1, ()).unwrap();

        Ok(Rc::new(Self {
            connection,
            globals,
            compositor,
            shm,
            xdg_decoration_manager,
        }))
    }

    pub fn connection(&self) -> Arc<Connection> {
        self.connection.clone()
    }
}

impl PlatformApplication for Application {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Dispatch<WlRegistry, GlobalListContents> for State {
    fn event(
        _: &mut State,
        _: &WlRegistry,
        _: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
    }
}

// impl Dispatch<XdgWmBase, ()> for State {
//     fn event(
//         _: &mut Self,
//         wm_base: &XdgWmBase,
//         event: xdg_wm_base::Event,
//         _: &(),
//         _: &Connection,
//         _: &QueueHandle<Self>,
//     ) {
//         if let xdg_wm_base::Event::Ping { serial } = event {
//             wm_base.pong(serial);
//         }
//     }
// }
