use crate::node::{create_node, Node};
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Tree<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    last_node: Option<Rc<RefCell<Node<T>>>>,
    pub size: usize,
}

impl<T: Debug + Clone + Copy + PartialEq> Tree<T> {
    pub fn new(val: Option<T>) -> Self {
        let (head, last_node) = match val {
            Some(v) => {
                let new_node = Rc::new(RefCell::new(Node::new(v)));
                (Some(Rc::clone(&new_node)), Some(new_node))
            }
            None => (None, None),
        };

        Self {
            head,
            last_node,
            size: 0,
        }
    }

    pub fn append_last(&mut self, val: T) {
        let new_node = create_node(val);
        self.set_last(&new_node);
    }

    pub fn add_child(&mut self, val: T) {
        match &self.last_node {
            Some(last) => {
                let mut last_node = last.borrow_mut();
                let new_node_created = create_node(val);
                last_node.add_child(Rc::clone(&new_node_created));
                let mut new_node_mut = new_node_created.borrow_mut();
                new_node_mut.parent = Some(Rc::downgrade(&last));
            }
            None => println!("The last node not exist! Maybe the tree does not have nodes"),
        }
    }

    pub fn remove_last(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        match self.last_node.take() {
            Some(last) => {
                if let Some(parent) = &last.borrow().parent {
                    self.last_node = Weak::upgrade(&parent);
                }
                self.size -= 1;
                Some(last)
            }
            None => None,
        }
    }

    pub fn set_last(&mut self, new_node: &Rc<RefCell<Node<T>>>) {
        if self.last_node.is_none() {
            self.last_node = Some(Rc::clone(new_node));
            self.head = Some(Rc::clone(&new_node));
            self.size += 1;
            return;
        }

        let last_node = self.last_node.take().unwrap();
        self.last_node = Some(Rc::clone(&new_node));

        new_node.borrow_mut().parent = Some(Rc::downgrade(&last_node));

        self.size += 1;
    }

    pub fn get_last_node(&self) -> Option<&Rc<RefCell<Node<T>>>> {
        self.last_node.as_ref()
    }
    pub fn get_last_value(&self) -> Option<T> {
        self.get_last_node().and_then(|v| Some(v.borrow().value))
    }
}
