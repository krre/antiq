use wayland_client::{
    Connection, Dispatch, QueueHandle, delegate_noop,
    globals::{GlobalList, GlobalListContents, registry_queue_init},
    protocol::{
        wl_compositor::WlCompositor,
        wl_registry::{self, WlRegistry},
        wl_shm::WlShm,
    },
};
use wayland_protocols::xdg::decoration::zv1::client::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1;

use crate::core::Result;
use crate::platform::PlatformApplication;

pub struct Application {
    pub(crate) connection: Connection,
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
    pub fn new() -> Result<Self> {
        let connection = Connection::connect_to_env()?;
        let (globals, queue) = registry_queue_init::<State>(&connection)?;
        let qh = queue.handle();
        let compositor: WlCompositor = globals.bind(&qh, 4..=5, ())?;
        let shm: WlShm = globals.bind(&qh, 1..=1, ())?;
        let xdg_decoration_manager: ZxdgDecorationManagerV1 = globals.bind(&qh, 1..=1, ())?;

        Ok(Self {
            connection,
            globals,
            compositor,
            shm,
            xdg_decoration_manager,
        })
    }
}

impl PlatformApplication for Application {}

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
