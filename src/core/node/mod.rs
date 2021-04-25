use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNT: AtomicUsize = AtomicUsize::new(0);

pub trait Node {
    fn id(&self) -> usize;

    fn add_child(&mut self, child: Rc<dyn Node>);

    fn remove_child(&mut self, index: usize);

    fn count(&self) -> usize;

    fn child_at(&self, index: usize) -> Option<Rc<dyn Node>>;

    fn set_parent(&mut self, parent: Option<Rc<dyn Node>>);

    fn parent(&self) -> Option<Rc<dyn Node>>;

    fn update(&mut self);
}

pub trait Update {
    fn update(&mut self);
}

pub struct UpdatedNode<T: Update> {
    id: usize,
    parent: Option<Rc<dyn Node>>,
    children: Vec<Rc<dyn Node>>,
    data: T,
}

impl<T: Update> Node for UpdatedNode<T> {
    fn id(&self) -> usize {
        self.id
    }

    fn add_child(&mut self, child: Rc<dyn Node>) {
        self.children.push(child);
    }

    fn remove_child(&mut self, index: usize) {
        self.children.remove(index);
    }

    fn count(&self) -> usize {
        self.children.len()
    }

    fn child_at(&self, index: usize) -> Option<Rc<dyn Node>> {
        if index > self.children.len() - 1 {
            None
        } else {
            Some(self.children[index].clone())
        }
    }

    fn set_parent(&mut self, parent: Option<Rc<dyn Node>>) {
        self.parent = parent;
    }

    fn parent(&self) -> Option<Rc<dyn Node>> {
        self.parent.clone()
    }

    fn update(&mut self) {
        self.data.update();
    }
}

impl<T: Update + Default> UpdatedNode<T> {
    pub fn new() -> Self {
        Self {
            id: next_id(),
            parent: None,
            children: Vec::new(),
            data: Default::default(),
        }
    }

    pub fn into_inner(self) -> T {
        self.data
    }

    pub fn get_ref(&self) -> &T {
        &self.data
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

fn next_id() -> usize {
    ID_COUNT.fetch_add(1, Ordering::SeqCst)
}
