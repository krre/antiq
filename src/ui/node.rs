use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub trait HasNodeState {
    fn node_state(&self) -> &NodeState;

    fn node_state_mut(&mut self) -> &mut NodeState;
}

pub trait Node: HasNodeState {
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

    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Node>>>) {
        self.node_state_mut().parent = parent;
    }

    fn parent(&self) -> Option<Weak<RefCell<dyn Node>>> {
        self.node_state().parent.clone()
    }

    fn update(&mut self) {}
}

pub struct NodeState {
    parent: Option<Weak<RefCell<dyn Node>>>,
    children: Vec<Rc<RefCell<dyn Node>>>,
}

impl NodeState {
    pub fn new() -> Self {
        Self {
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
