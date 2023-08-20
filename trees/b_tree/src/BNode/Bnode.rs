use std::{cell::RefCell, fmt::Display, rc::Rc};

pub type Link = Option<Rc<RefCell<BNode>>>;

#[derive(Debug)]
pub struct BNode {
    pub val: usize,
    pub left: Link,
    pub right: Link,
}

impl BNode {
    pub fn new(val: usize) -> Self {
        return BNode {
            val,
            left: None,
            right: None,
        };
    }

    pub fn new_link(val: usize) -> Link {
        return Some(Rc::new(RefCell::new(BNode {
            val,
            left: None,
            right: None,
        })));
    }
}

impl Display for BNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node Val: {:?}", self.val)
    }
}
