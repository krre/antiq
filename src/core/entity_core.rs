use super::Id;

#[derive(Default)]
pub struct EntityCore {
    id: Id,
}

impl EntityCore {
    pub fn id(&self) -> Id {
        self.id
    }
}
