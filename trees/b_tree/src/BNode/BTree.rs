use super::Bnode::{BNode, Link};

#[derive(Debug)]
pub struct BTree {
    pub head: Link,
}

impl BTree {
    pub fn add(&mut self, val: usize, node: &mut Link) -> Link {
        let mut node_val = node.clone().take();

        if let Some(inner_node) = node_val.take() {
            if val < inner_node.borrow().val {
                let new_node = BNode::new_link(val);
                inner_node.borrow_mut().left = new_node.clone();

                new_node
            } else {
                let new_node = BNode::new_link(val);
                inner_node.borrow_mut().right = new_node.clone();

                new_node
            }
        } else {
            node_val
        }
    }

    pub fn validate(&self) -> bool {
        Self::iterate(&self.head, 1000000, -1000000)
    }

    pub fn default(val: usize) -> Self {
        BTree {
            head: BNode::new_link(val),
        }
    }

    pub fn print_tree(&self) {
        BTree::recursvie_print(&self.head);
    }

    // Private
    fn recursvie_print(node: &Link) {
        if let Some(node) = node.clone().take() {
            println!("Val: {:?}", &node.borrow().val);

            if let Some(left) = &node.borrow().left {
                let copied = left.clone();
                BTree::recursvie_print(&Some(copied));
            }

            if let Some(right) = &node.borrow().right {
                let copied = right.clone();
                BTree::recursvie_print(&Some(copied));
            }
        }
    }

    /*
     *
     * Base Cases:
     * If node !exist -> true;
     * If node.val > min && node.val < max,
     * and all children also follow: true;
     *
     * pre -> convert usize -> i32
     */
    fn iterate(node: &Link, max: i32, min: i32) -> bool {
        if let Some(e_node) = node.clone().take() {
            let local_compare_val = e_node.borrow().val as i32;

            local_compare_val > min
                && local_compare_val < max
                && BTree::iterate(&e_node.borrow().left, e_node.borrow().val as i32, min)
                && BTree::iterate(&e_node.borrow().right, max, e_node.borrow().val as i32)
        } else {
            true
        }
    }
}
