use std::{
    fs::{DirBuilder, File},
    io::{Read, Write},
    path::PathBuf,
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use super::Context;

pub struct Preferences<T: Default + Serialize + for<'a> Deserialize<'a>> {
    data: T,
    is_loaded: bool,
    context: Rc<Context>,
}

impl<T: Default + Serialize + for<'a> Deserialize<'a>> Preferences<T> {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            data: T::default(),
            is_loaded: false,
            context,
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

    pub fn load(&mut self) {
        if let Ok(mut file) = File::open(self.path()) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            self.data = serde_json::from_slice::<T>(contents.as_bytes()).unwrap();
            self.is_loaded = true;
        } else {
            self.is_loaded = false;
        }
    }

    pub fn save(&self) {
        let value = serde_json::to_string(&self.data).unwrap();

        DirBuilder::new()
            .recursive(true)
            .create(self.dir())
            .unwrap();

        let mut file = File::create(self.path()).unwrap();
        file.write_all(value.as_bytes()).unwrap();
    }

    pub fn dir(&self) -> PathBuf {
        [
            dirs::config_dir().unwrap(),
            self.context.organization.clone().into(),
        ]
        .iter()
        .collect()
    }

    pub fn path(&self) -> PathBuf {
        let mut result: PathBuf = [self.dir(), self.context.name.clone().into()]
            .iter()
            .collect();
        result.set_extension("prefs");
        result
    }
}
