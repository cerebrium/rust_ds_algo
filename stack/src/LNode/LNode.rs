use std::fmt::Display;

pub type Link = Option<Box<LNode>>;

#[derive(Debug)]
pub struct LNode {
    pub val: usize,
    pub next: Link,
}

impl Display for LNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Linked Node: {} ", self.val)
    }
}

impl LNode {
    pub fn new(val: usize) -> Self {
        LNode { val, next: None }
    }
}
