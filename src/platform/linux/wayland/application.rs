use std::{any::Any, rc::Rc, sync::Arc};
use wayland_client::{
    delegate_noop, globals::{registry_queue_init, GlobalListContents}, protocol::{wl_compositor::WlCompositor, wl_registry::{self, WlRegistry}}, Connection, QueueHandle
};

use crate::platform::{PlatformApplication, PlatformEventLoop};

pub struct Application {
    pub(crate) connection: Arc<Connection>,
    pub(crate) compositor: WlCompositor,
}

struct State {}

delegate_noop!(State: ignore WlCompositor);

impl Application {
    pub fn new() -> Result<Rc<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        let connection = Arc::new(Connection::connect_to_env()?);
        let (globals, queue) = registry_queue_init::<State>(&connection).unwrap();
        let compositor: WlCompositor = globals.bind(&queue.handle(), 4..=5, ()).unwrap();

        Ok(Rc::new(Self { connection, compositor }))
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