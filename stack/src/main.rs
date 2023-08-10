use crate::LNode::Stack::Stack;

pub mod LNode;

fn main() {
    let mut stack = Stack::default();
    stack.push(0);
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);

    let returned_val = stack.pop();
    println!(" value: {:?}", returned_val);
}
