fn main() {
    let nums: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let found_true: bool = binary_search(&nums, &6);
    let high_bound_found: bool = binary_search(&nums, &10);
    let low_found_found: bool = binary_search(&nums, &1);
    let not_found: bool = binary_search(&nums, &20);

    println!(
        "found_true: {:?} \n high_bound_true: {:?} \n low_bound_true: {:?} \n not_found: {:?}",
        found_true, high_bound_found, low_found_found, not_found
    );
}

fn binary_search(nums: &Vec<usize>, target: &usize) -> bool {
    let mut high: usize = nums.len();
    let mut low: usize = 0;

    while low < high {
        // Rust integers already round down
        let mid = low + (high - low) / 2;
        let mid_value = nums.get(mid);

        match mid_value {
            Some(mid_value) if mid_value == target => {
                return true;
            }
            Some(mid_value) if mid_value > target => {
                // Higher than mid, look lower
                high = mid;
            }
            Some(mid_value) if mid_value < target => {
                // Lower than mid look higher
                low = mid + 1;
            }
            None => {
                return false;
            }
            Some(_) => {
                return false;
            }
        }
    }

    false
}
