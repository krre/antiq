use std::sync::atomic::{AtomicUsize, Ordering};

static NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
pub enum NodeKind<T = ()> {
    Node,
    Widget,
    Layout,
    User(T),
}

pub trait NodeLike {
    fn kind() -> NodeKind;
}

#[derive(Clone, Copy, PartialEq)]
pub struct NodeId(usize);

pub struct Node<T: NodeLike = ()> {
    id: NodeId,
    kind: NodeKind,
    data: Box<T>,
    children: Vec<Node<T>>,
    parent: Option<NodeId>,
}

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

impl<T: NodeLike> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            id: NodeId::generate(),
            kind: T::kind(),
            data: Box::new(data),
            children: Vec::new(),
            parent: None,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn kind(&self) -> NodeKind {
        self.kind
    }

    pub fn data(&self) -> &Box<T> {
        &self.data
    }

    pub fn add_child(&mut self, child: Node<T>) {
        self.children.push(child);
    }

    pub fn insert_child(&mut self, index: usize, child: Node<T>) {
        if index > self.children.len() {
            self.add_child(child);
        } else {
            self.children.insert(index, child);
        }
    }

    pub fn remove_child(&mut self, index: usize) {
        if index < self.children.len() {
            self.children.remove(index);
        }
    }

    pub fn child_at(&self, index: usize) -> Option<&Node<T>> {
        if index > self.len() - 1 {
            None
        } else {
            Some(&self.children[index])
        }
    }

    pub fn find_child(&self, id: NodeId) -> Option<&Node<T>> {
        for child in self.children.iter() {
            if child.id == id {
                return Some(child);
            }

            let nested_child = child.find_child(id);

            if nested_child.is_some() {
                return nested_child;
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn set_parent(&mut self, parent: Option<NodeId>) {
        self.parent = parent;
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }
}

impl NodeLike for () {
    fn kind() -> NodeKind {
        NodeKind::Node
    }
}
