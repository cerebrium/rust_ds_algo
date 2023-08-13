use std::{cell::RefCell, fmt::Display, rc::Rc};

pub type Link = Option<Rc<RefCell<LNode>>>;

#[derive(Debug)]
pub struct LNode {
    pub val: usize,
    pub next: Link,
    pub prev: Link,
}

impl LNode {
    pub fn new(val: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(LNode {
            val,
            next: None,
            prev: None,
        }))
    }
}

impl Display for LNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node val: {}", self.val)
    }
}
