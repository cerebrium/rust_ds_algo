use std::fmt::Display;
use std::rc::Rc;

use super::LNode::{LNode, Link};

#[derive(Debug)]
pub struct Que {
    tail: Link,
    head: Link,
}

impl Default for Que {
    fn default() -> Self {
        Self {
            tail: None,
            head: None,
        }
    }
}

impl Display for Que {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node;
        node = self.tail.clone();

        while let Some(p_node) = node.take() {
            let val = &p_node.borrow().val;
            node = p_node.borrow().prev.clone();

            write!(f, " {}", val);
        }
        Ok(())
    }
}

impl Que {
    pub fn enque(&mut self, val: usize) {
        let new_node = LNode::new(val);

        match self.head.take() {
            Some(old_head) => {
                // new_node -> head;
                old_head.borrow_mut().prev = Some(new_node.clone());

                // new node points towards current head;
                new_node.borrow_mut().next = Some(old_head);

                // head -> old_head;
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn deque(&mut self) -> Option<usize> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val
        })
    }
}
