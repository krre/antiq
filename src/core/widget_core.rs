use super::EntityCore;

#[derive(Default)]
pub struct WidgetCore {
    entity_core: EntityCore,
}

impl WidgetCore {
    pub fn entity_ref(&self) -> &EntityCore {
        &self.entity_core
    }

    pub fn entity_mut(&mut self) -> &mut EntityCore {
        &mut self.entity_core
    }
}
