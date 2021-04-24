use std::rc::Rc;

pub trait Update {
    fn update(&mut self);
}

pub struct Node<T: Update + Default> {
    parent: Option<Rc<Node<T>>>,
    children: Vec<Rc<Node<T>>>,
    content: T,
}

impl<T: Update + Default> Node<T> {
    pub fn new() -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            content: Default::default(),
        }
    }

    pub fn get(&self) -> &T {
        &self.content
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.content
    }

    pub fn add_child(&mut self, child: Rc<Node<T>>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, index: usize) {
        self.children.remove(index);
    }

    pub fn count(&self) -> usize {
        self.children.len()
    }

    pub fn child_at(&self, index: usize) -> Option<Rc<Node<T>>> {
        if index > self.children.len() - 1 {
            None
        } else {
            Some(self.children[index].clone())
        }
    }

    pub fn set_parent(&mut self, parent: Option<Rc<Node<T>>>) {
        self.parent = parent;
    }

    pub fn parent(&self) -> Option<Rc<Node<T>>> {
        self.parent.clone()
    }

    pub fn update(&mut self) {
        self.content.update();
    }
}
