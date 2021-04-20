use std::rc::Rc;

pub struct Node {
    parent: Option<Rc<Node>>,
    children: Vec<Rc<Node>>,
}

impl Node {
    pub fn add_child(&mut self, child: Rc<Node>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, index: usize) {
        self.children.remove(index);
    }

    pub fn count(&self) -> usize {
        self.children.len()
    }

    pub fn child_at(&self, index: usize) -> Option<Rc<Node>> {
        if index > self.children.len() - 1 {
            None
        } else {
            Some(self.children[index].clone())
        }
    }

    pub fn set_parent(&mut self, parent: Option<Rc<Node>>) {
        self.parent = parent;
    }

    pub fn parent(&self) -> Option<Rc<Node>> {
        self.parent.clone()
    }
}
