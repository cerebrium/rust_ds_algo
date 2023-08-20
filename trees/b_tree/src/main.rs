use crate::BNode::BTree::BTree;

mod BNode;

fn main() {
    let mut b_tree = BTree::default(20);
    make_tree(&mut b_tree);
    b_tree.print_tree();
    let is_valid = b_tree.validate();
    println!("is this valid: {:?}", is_valid);
}

fn make_tree(b_tree: &mut BTree) {
    let mut head = b_tree.head.clone().take();

    // left
    let mut head_left = b_tree.add(10, &mut head);
    b_tree.add(5, &mut head_left);
    b_tree.add(15, &mut head_left);

    // right
    let mut head_right = b_tree.add(30, &mut head);
    b_tree.add(25, &mut head_right);
    b_tree.add(35, &mut head_right);
}
