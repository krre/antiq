use std::{
    fs::{DirBuilder, File},
    io::{Read, Write},
    marker::PhantomData,
    path::PathBuf,
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use super::Context;

pub enum Format {
    Compact,
    Pretty,
}

pub struct Preferences<T: Default + Serialize + for<'a> Deserialize<'a>> {
    format: Format,
    data: T,
    is_loaded: bool,
    context: Rc<Context>,
}

pub struct PreferencesBuilder<T: Default + Serialize + for<'a> Deserialize<'a>> {
    format: Format,
    context: Rc<Context>,
    data: PhantomData<T>,
}

impl<T: Default + Serialize + for<'a> Deserialize<'a>> Preferences<T> {
    pub fn new(context: Rc<Context>) -> Self {
        let builder = PreferencesBuilder::<T>::new(context);
        builder.build()
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
        }
    }

    pub fn save(&self) {
        let value = match self.format {
            Format::Compact => serde_json::to_string(&self.data),
            Format::Pretty => serde_json::to_string_pretty(&self.data),
        };

        DirBuilder::new()
            .recursive(true)
            .create(self.dir())
            .unwrap();

        let mut file = File::create(self.path()).unwrap();
        file.write_all(value.unwrap().as_bytes()).unwrap();
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

impl<T: Default + Serialize + for<'a> Deserialize<'a>> PreferencesBuilder<T> {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            format: Format::Compact,
            context,
            data: PhantomData,
        }
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    pub fn build(self) -> Preferences<T> {
        Preferences {
            format: self.format,
            context: self.context.clone(),
            is_loaded: false,
            data: T::default(),
        }
    }
}
