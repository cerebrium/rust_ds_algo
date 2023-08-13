use std::fmt::Display;

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

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut curr = &self.head;

        while let Some(p_node) = curr {
            let curr_v = p_node.val;
            curr = &p_node.next;

            write!(f, "{}", curr_v);
        }

        Ok(())
    }
}
