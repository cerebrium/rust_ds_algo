use crate::LNode::Que::Que;

pub mod LNode;

fn main() {
    let mut que = Que::default();

    que.enque(2);
    que.enque(3);
    que.enque(4);
    que.enque(5);
    println!("que: {}", que);

    que.deque();
    println!("que: {}", que)
}
