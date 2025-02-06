use std::{any::Any, os::fd::OwnedFd, rc::Rc, sync::Arc};
use wayland_backend::client::Backend;
use wayland_client::{
    backend,
    protocol::{wl_display, wl_registry},
    Connection, Proxy,
};

use crate::platform::{PlatformApplication, PlatformEventLoop};

pub struct Application {
    pub(crate) connection: Arc<Connection>,
    pub(crate) registry: wl_registry::WlRegistry
}

struct RegistryData(Arc<Connection>);

impl Application {
    pub fn new() -> Result<Rc<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        let connection = Arc::new(Connection::connect_to_env()?);
        let display = connection.display();
        let registry_data = Arc::new(RegistryData(connection.clone()));

        let registry: wl_registry::WlRegistry = display
            .send_constructor(wl_display::Request::GetRegistry {}, registry_data.clone())
            .unwrap();

        connection.roundtrip().unwrap();

        Ok(Rc::new(Self { connection, registry }))
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

impl backend::ObjectData for RegistryData {
    fn event(
        self: Arc<Self>,
        _: &Backend,
        msg: backend::protocol::Message<backend::ObjectId, OwnedFd>,
    ) -> Option<Arc<dyn backend::ObjectData>> {
        // Here, we parse the wire message into an event using Proxy::parse_event.
        let (_registry, event) = wl_registry::WlRegistry::parse_event(&self.0, msg).unwrap();

        // Similar to the dispatch example, we only care about the global event and
        // will print out the received globals.
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            println!("[{}] {} (v{})", name, interface, version);
        }
        None
    }

    // This method is called whenever the object is destroyed. In the case of our registry,
    // however, there is no way to destroy it, so we will mark it as unreachable.
    fn destroyed(&self, _: wayland_backend::client::ObjectId) {
        unreachable!();
    }
}
