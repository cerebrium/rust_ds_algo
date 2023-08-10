use crate::l_list::LNode::LNode;

mod l_list;

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut linked_head = LNode::create_linked_list(nums);

    match &linked_head {
        Ok(node) => LNode::print_list(&node),
        Err(_) => panic!("lined_head panic!"),
    }

    println!("Delete");

    match &mut linked_head {
        Ok(node) => LNode::delete_next_node(node, 5),
        Err(_) => panic!("lined_head panic!"),
    }
}
