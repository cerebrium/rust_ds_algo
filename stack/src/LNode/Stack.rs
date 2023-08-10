use super::LNode::{LNode, Link};

#[derive(Debug)]
pub struct Stack {
    head: Link,
}

impl Stack {
    pub fn push(&mut self, val: usize) {
        let new_node = Box::new(LNode {
            val,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<usize> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self { head: None }
    }
}
