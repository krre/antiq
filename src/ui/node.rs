use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::atomic::{AtomicUsize, Ordering},
};

static NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq)]
pub struct NodeId(usize);

impl NodeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn generate() -> Self {
        Self(NODE_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

pub trait HasNodeState {
    fn node_state(&self) -> &NodeState;

    fn node_state_mut(&mut self) -> &mut NodeState;
}

pub trait Node: HasNodeState {
    fn id(&self) -> NodeId {
        self.node_state().id
    }

    fn add_child(&mut self, child: Rc<RefCell<dyn Node>>) {
        self.node_state_mut().children.push(child);
    }

    fn insert_child(&mut self, index: usize, child: Rc<RefCell<dyn Node>>) {
        if index > self.count() {
            self.add_child(child);
        } else {
            self.node_state_mut().children.insert(index, child);
        }
    }

    fn remove_child(&mut self, index: usize) {
        if index < self.count() {
            self.node_state_mut().children.remove(index);
        }
    }

    fn count(&self) -> usize {
        self.node_state().children.len()
    }

    fn child_at(&self, index: usize) -> Option<Weak<RefCell<dyn Node>>> {
        if index > self.count() - 1 {
            None
        } else {
            Some(Rc::downgrade(&self.node_state().children[index]).clone())
        }
    }

    fn find_child(&self, id: NodeId) -> Option<Weak<RefCell<dyn Node>>> {
        for child in self.node_state().children.iter() {
            if child.borrow().id() == id {
                return Some(Rc::downgrade(&child.clone()));
            }

            let nested_child = child.borrow().find_child(id);

            if nested_child.is_some() {
                return nested_child;
            }
        }

        None
    }

    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Node>>>) {
        self.node_state_mut().parent = parent;
    }

    fn parent(&self) -> Option<Weak<RefCell<dyn Node>>> {
        self.node_state().parent.clone()
    }

    fn update(&mut self) {}
}

pub struct NodeState {
    id: NodeId,
    parent: Option<Weak<RefCell<dyn Node>>>,
    children: Vec<Rc<RefCell<dyn Node>>>,
}

impl NodeState {
    pub fn new() -> Self {
        Self {
            id: NodeId::generate(),
            parent: None,
            children: Vec::new(),
        }
    }
}

impl Default for NodeState {
    fn default() -> Self {
        Self::new()
    }
}
