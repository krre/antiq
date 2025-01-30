use std::{
    fs::{DirBuilder, File},
    io::{Read, Write},
    marker::PhantomData,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use super::application::Application;

pub trait DSD: Default + Serialize + for<'a> Deserialize<'a> {}

pub enum Format {
    Compact,
    Pretty,
}

pub struct Preferences<T: DSD> {
    dir: PathBuf,
    path: PathBuf,
    format: Format,
    data: T,
    is_loaded: bool,
}

pub struct PreferencesBuilder<T: DSD> {
    dir: PathBuf,
    path: PathBuf,
    format: Format,
    data: PhantomData<T>,
}

impl<T: DSD> Preferences<T> {
    pub fn new(application: &Application) -> Self {
        let builder = PreferencesBuilder::<T>::new(application);
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
        self.dir.clone()
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl<T: DSD> PreferencesBuilder<T> {
    pub fn new(application: &Application) -> Self {
        let dir: PathBuf = [
            dirs::config_dir().unwrap(),
            application.organization().into(),
        ]
        .iter()
        .collect();

        let mut path: PathBuf = [dir.clone(), application.name().into()].iter().collect();
        path.set_extension("prefs");

        Self {
            dir,
            path,
            format: Format::Compact,
            data: PhantomData,
        }
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    pub fn build(self) -> Preferences<T> {
        Preferences {
            dir: self.dir,
            path: self.path,
            format: self.format,
            is_loaded: false,
            data: T::default(),
        }
    }
}

impl<T> DSD for T where T: Default + Serialize + for<'a> Deserialize<'a> {}
