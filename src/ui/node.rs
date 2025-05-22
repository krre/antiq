use std::rc::Rc;

pub trait Node {
    fn add_child(&mut self, child: Rc<dyn Node>);

    fn remove_child(&mut self, index: usize);

    fn count(&self) -> usize;

    fn child_at(&self, index: usize) -> Option<Rc<dyn Node>>;

    fn set_parent(&mut self, parent: Option<Rc<dyn Node>>);

    fn parent(&self) -> Option<Rc<dyn Node>>;

    fn update(&mut self);
}

#[derive(Default)]
pub struct NodeState {
    parent: Option<Rc<dyn Node>>,
    children: Vec<Rc<dyn Node>>,
}

impl Node for NodeState {
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

    fn update(&mut self) {}
}
