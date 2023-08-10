use super::LNode::{LNode, Link};

#[derive(Debug)]
pub struct Que {
    head: Link,
    tail: Link,
}

impl Que {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn enque(&mut self, val: usize) {
        let new_node = Box::new(LNode {
            val,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn deque(&mut self) -> Option<usize> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
}
