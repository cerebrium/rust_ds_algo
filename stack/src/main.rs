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

    println!("current stack: {}", stack);

    let returned_val = stack.pop();
    println!("did pop");
    println!("current stack: {}", stack);

    if let Some(returned_val) = returned_val {
        println!(" returned value: {:?}", returned_val);
    }
}
