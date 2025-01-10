use serde::{Deserialize, Serialize};

pub trait SD: Default + Serialize + for<'a> Deserialize<'a> {}

pub struct Preferences<T: SD> {
    data: T,
    is_loaded: bool,
}

impl<T: SD> Preferences<T> {
    pub fn new() -> Self {
        Self {
            data: T::default(),
            is_loaded: false,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    pub fn get_ref(&self) -> &T {
        &self.data
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }
}
