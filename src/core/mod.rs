pub mod application;
pub mod log;
pub mod window;

mod id;

pub use id::Id;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait UpgradeOrErr<T> {
    fn upgrade_or_err(self) -> std::result::Result<T, &'static str>;
}

impl<T> UpgradeOrErr<std::rc::Rc<T>> for std::rc::Weak<T> {
    fn upgrade_or_err(self) -> std::result::Result<std::rc::Rc<T>, &'static str> {
        self.upgrade().ok_or("Window weak reference is invalid")
    }
}
