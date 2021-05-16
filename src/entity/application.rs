use crate::core::Id;
use crate::entity::Entity;

pub struct Application {
    id: Id,
}

impl Entity for Application {
    fn id(&self) -> Id {
        self.id
    }
}
