use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
    usize,
};

static NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub trait NodeLike: Any {
    fn id(&self) -> NodeId;

    fn add_child(&mut self, child: Box<dyn NodeLike>);

    fn insert_child(&mut self, index: usize, child: Box<dyn NodeLike>);

    fn remove_child(&mut self, index: usize);

    fn find_child(&self, id: NodeId) -> Option<&Box<dyn NodeLike>> {
        for child in self.children().iter() {
            if child.id() == id {
                return Some(child);
            }

            let nested_child = child.find_child(id);

            if nested_child.is_some() {
                return nested_child;
            }
        }

        None
    }

    fn children(&self) -> &[Box<dyn NodeLike>];

    fn set_parent(&mut self, parent: Box<dyn NodeLike>);

    fn parent(&self) -> Option<&Box<dyn NodeLike>>;
}

#[derive(Clone, Copy, PartialEq)]
pub struct NodeId(usize);

impl NodeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn generate() -> Self {
        Self(NODE_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

pub struct Node {
    id: NodeId,
    children: Vec<Box<dyn NodeLike>>,
    parent: Option<Box<dyn NodeLike>>,
}

impl NodeLike for Node {
    fn id(&self) -> NodeId {
        self.id
    }

    fn add_child(&mut self, child: Box<dyn NodeLike>) {
        self.children.push(child);
    }

    fn insert_child(&mut self, index: usize, child: Box<dyn NodeLike>) {
        self.children.insert(index, child);
    }

    fn remove_child(&mut self, index: usize) {
        self.children.remove(index);
    }

    fn find_child(&self, id: NodeId) -> Option<&Box<dyn NodeLike>> {
        for child in self.children().iter() {
            if child.id() == id {
                return Some(child);
            }

            let nested_child = child.find_child(id);

            if nested_child.is_some() {
                return nested_child;
            }
        }

        None
    }

    fn children(&self) -> &[Box<dyn NodeLike>] {
        &self.children
    }

    fn set_parent(&mut self, parent: Box<dyn NodeLike>) {
        self.parent = Some(parent);
    }

    fn parent(&self) -> Option<&Box<dyn NodeLike>> {
        self.parent.as_ref()
    }
}
