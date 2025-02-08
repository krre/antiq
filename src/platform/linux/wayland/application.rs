use std::{any::Any, rc::Rc, sync::Arc};
use wayland_client::{
    delegate_noop, globals::{registry_queue_init, GlobalListContents}, protocol::{wl_compositor::WlCompositor, wl_registry::{self, WlRegistry}, wl_shm::WlShm}, Connection, QueueHandle
};
use wayland_protocols::xdg::shell::client::xdg_wm_base::XdgWmBase;

use crate::platform::{PlatformApplication, PlatformEventLoop};

pub struct Application {
    pub(crate) connection: Arc<Connection>,
    pub(crate) compositor: WlCompositor,
    pub(crate) shm: WlShm,
    pub(crate) xdg_wm_base: XdgWmBase
}

struct State {}

delegate_noop!(State: ignore WlCompositor);
delegate_noop!(State: ignore WlShm);
delegate_noop!(State: ignore XdgWmBase);

impl Application {
    pub fn new() -> Result<Rc<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        let connection = Arc::new(Connection::connect_to_env()?);
        let (globals, queue) = registry_queue_init::<State>(&connection).unwrap();
        let qh = queue.handle();
        let compositor: WlCompositor = globals.bind(&qh, 4..=5, ()).unwrap();
        let shm: WlShm = globals.bind(&qh, 1..=1, ()).unwrap();
        let xdg_wm_base: XdgWmBase = globals.bind(&qh, 5..=6, ()).unwrap();

        Ok(Rc::new(Self { connection, compositor, shm, xdg_wm_base }))
    }

    pub fn connection(&self) -> Arc<Connection> {
        self.connection.clone()
    }
}

impl PlatformApplication for Application {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn init(&self, event_loop: Rc<dyn PlatformEventLoop>) {}
}

impl wayland_client::Dispatch<WlRegistry, GlobalListContents> for State {
    fn event(
        state: &mut State,
        proxy: &WlRegistry,
        event: wl_registry::Event,
        data: &GlobalListContents,
        conn: &Connection,
        qhandle: &QueueHandle<State>,
    ) {

    }
}