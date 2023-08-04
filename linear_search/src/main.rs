fn main() {
    let nums: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let found = find_element(nums, 11);
    println!("is the element found: {:?}", found);
}

fn find_element(arr: Vec<usize>, target: usize) -> bool {
    for i in 0..arr.len() {
        if i == target {
            return true;
        }
    }
    false
}
