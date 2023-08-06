use crate::l_list::LNode::LNode;

mod l_list;

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let linked_head = LNode::create_linked_list(nums);

    match linked_head {
        Ok(node) => LNode::print_list(node),
        Err(_) => panic!("lined_head panic!"),
    }
}
