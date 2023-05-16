use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

pub fn create_node<T: Debug + PartialEq>(val: T) -> Rc<RefCell<Node<T>>> {
    Rc::new(RefCell::new(Node::new(val)))
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    children: Vec<Rc<RefCell<Node<T>>>>,
    pub parent: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Debug + PartialEq> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: vec![],
            parent: None,
        }
    }
    pub fn is_leaf(&self) -> bool {
        self.children_len() == 0
    }
    pub fn add_child(&mut self, child_node: Rc<RefCell<Node<T>>>) {
        self.children.push(child_node)
    }
    pub fn children_len(&self) -> usize {
        self.children.len()
    }
    pub fn get_child(&self, i: usize) -> Option<&Rc<RefCell<Node<T>>>> {
        self.children.get(i)
    }
    pub fn remove_child(&mut self, value: T) {
        self.children = self
            .children
            .iter()
            .filter(|v| v.borrow().value != value)
            .map(|v| Rc::clone(&v))
            .collect()
    }
}
