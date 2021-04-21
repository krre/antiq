use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub trait Updatable {
    fn update(&mut self);
}

pub struct Node<T: Updatable + Default> {
    parent: Option<Rc<Node<T>>>,
    children: Vec<Rc<Node<T>>>,
    content: T,
}

impl<T: Updatable + Default> Node<T> {
    pub fn new() -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            content: Default::default(),
        }
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

impl<T: Updatable + Default> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<T: Updatable + Default> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
