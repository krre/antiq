use std::rc::{Rc, Weak};

pub trait HasNodeState {
    fn node_state(&self) -> &NodeState;

    fn node_state_mut(&mut self) -> &mut NodeState;
}

pub trait Node: HasNodeState {
    fn add_child(&mut self, child: Rc<dyn Node>) {
        self.node_state_mut().children.push(child);
    }

    fn remove_child(&mut self, index: usize) {
        if index < self.count() {
            self.node_state_mut().children.remove(index);
        }
    }

    fn count(&self) -> usize {
        self.node_state().children.len()
    }

    fn child_at(&self, index: usize) -> Option<Rc<dyn Node>> {
        if index > self.count() - 1 {
            None
        } else {
            Some(self.node_state().children[index].clone())
        }
    }

    fn set_parent(&mut self, parent: Option<Weak<dyn Node>>) {
        self.node_state_mut().parent = parent;
    }

    fn parent(&self) -> Option<Weak<dyn Node>> {
        self.node_state().parent.clone()
    }

    fn update(&mut self) {}
}

pub struct NodeState {
    parent: Option<Weak<dyn Node>>,
    children: Vec<Rc<dyn Node>>,
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
