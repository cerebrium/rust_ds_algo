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

    pub fn print_tree_pre_order(&self) -> Vec<usize> {
        let mut nums = vec![];
        BTree::recursvie_print(&self.head, &mut nums);
        nums
    }

    pub fn print_tree_in_order(&self) -> Vec<usize> {
        let mut nums = vec![];
        BTree::recursvie_print_in_order(&self.head, &mut nums);
        nums
    }

    pub fn print_tree_in_post_order(&self) -> Vec<usize> {
        let mut nums = vec![];
        BTree::recursive_print_post_order(&self.head, &mut nums);
        nums
    }

    pub fn compare_b_tree(&self, b: &BTree) -> bool {
        /*
         *
         * Do a in order recursive check
         * on each node;
         *
         */

        BTree::cmp_a_b(&self.head, &b.head)
    }
    // Private
    fn recursvie_print(node: &Link, nums: &mut Vec<usize>) {
        if let Some(node) = node.clone().take() {
            nums.push(node.borrow().val);
            if let Some(left) = &node.borrow().left {
                let copied = left.clone();
                BTree::recursvie_print(&Some(copied), nums);
            }

            if let Some(right) = &node.borrow().right {
                let copied = right.clone();
                BTree::recursvie_print(&Some(copied), nums);
            }
        }
    }

    fn recursvie_print_in_order(node: &Link, nums: &mut Vec<usize>) {
        if let Some(node) = node.clone().take() {
            if let Some(left) = &node.borrow().left {
                let copied = left.clone();
                BTree::recursvie_print_in_order(&Some(copied), nums)
            }

            nums.push(node.borrow().val);

            if let Some(right) = &node.borrow().right {
                let copied = right.clone();
                BTree::recursvie_print_in_order(&Some(copied), nums)
            }
        }
    }

    fn recursive_print_post_order(node: &Link, nums: &mut Vec<usize>) {
        if let Some(node) = node.clone().take() {
            if let Some(left) = &node.borrow().left {
                let copied = left.clone();
                BTree::recursive_print_post_order(&Some(copied), nums)
            }

            if let Some(right) = &node.borrow().right {
                let copied = right.clone();
                BTree::recursive_print_post_order(&Some(copied), nums)
            }

            nums.push(node.borrow().val);
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
                && BTree::iterate(&e_node.borrow().left, local_compare_val, min)
                && BTree::iterate(&e_node.borrow().right, max, local_compare_val)
        } else {
            true
        }
    }

    fn cmp_a_b(a: &Link, b: &Link) -> bool {
        /*
        *
        *Base cases:
        1. If both nodes are None; -> true
        2. if either node is None; -> false
        3. if node.a.val == node.b.val; -> true
        else -> false
        *
        */

        let is_equal = if let Some(node_a) = a {
            if let Some(node_b) = b {
                if node_b.borrow().val == node_a.borrow().val {
                    BTree::cmp_a_b(&node_a.clone().borrow().left, &node_b.clone().borrow().left)
                        && BTree::cmp_a_b(
                            &node_a.clone().borrow().right,
                            &node_b.clone().borrow().right,
                        )
                } else {
                    false
                }
            } else {
                false
            }
        } else if let Some(node_b) = b {
            false
        } else {
            true
        };

        is_equal
    }
}
