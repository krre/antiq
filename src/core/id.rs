use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Id(usize);

impl Id {
    pub fn inner(&self) -> usize {
        self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self(ID_COUNT.fetch_add(1, Ordering::SeqCst))
    }
}
