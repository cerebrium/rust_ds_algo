use crate::LNode::Que::Que;

pub mod LNode;

fn main() {
    let mut stack = Que::new();
    stack.enque(0);
    stack.enque(1);
    stack.enque(2);
    stack.enque(3);
    stack.enque(4);
    stack.enque(5);

    let returned_val = stack.deque();
    println!("destack value: {:?}", returned_val);
}
