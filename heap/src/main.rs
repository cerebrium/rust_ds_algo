use crate::Heap::MinHeap::MinHeap;

pub mod Heap;

fn main() {
    let mut heap = MinHeap::default();

    let mut curr = 100;
    loop {
        if (curr == 0) {
            break;
        }
        heap.add(curr);
        curr -= 1;
    }

    for num in 1..10 {
        let res = heap.pop();

        if let Some(res_v) = res {
            let is_equal = num as usize == res_v;
            println!("is equal: {:?} {:?} {:?}", is_equal, num, res_v);
        } else {
            println!("No Value");
        }
    }
}
