use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
    usize,
};

static NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub trait Node: Any {
    fn id(&self) -> NodeId;

    fn add_child(&mut self, child: Box<dyn Node>);

    fn insert_child(&mut self, index: usize, child: Box<dyn Node>);

    fn remove_child(&mut self, index: usize);

    fn find_child(&self, id: NodeId) -> Option<&Box<dyn Node>> {
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

    fn children(&self) -> &[Box<dyn Node>];

    fn set_parent(&mut self, parent: Box<dyn Node>);

    fn parent(&self) -> Option<&Box<dyn Node>>;
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

pub struct NodeData {
    id: NodeId,
    children: Vec<Box<dyn Node>>,
    parent: Option<Box<dyn Node>>,
}

impl NodeData {
    pub fn new() -> Self {
        Self::with_id(NodeId::generate())
    }

    pub fn with_id(id: NodeId) -> Self {
        Self {
            id,
            children: Vec::new(),
            parent: None,
        }
    }
}

impl Node for NodeData {
    fn id(&self) -> NodeId {
        self.id
    }

    fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.push(child);
    }

    fn insert_child(&mut self, index: usize, child: Box<dyn Node>) {
        self.children.insert(index, child);
    }

    fn remove_child(&mut self, index: usize) {
        self.children.remove(index);
    }

    fn find_child(&self, id: NodeId) -> Option<&Box<dyn Node>> {
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

    fn children(&self) -> &[Box<dyn Node>] {
        &self.children
    }

    fn set_parent(&mut self, parent: Box<dyn Node>) {
        self.parent = Some(parent);
    }

    fn parent(&self) -> Option<&Box<dyn Node>> {
        self.parent.as_ref()
    }
}
