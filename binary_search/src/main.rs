fn main() {
    let nums: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let found_true: bool = binary_search(&nums, 6);
    let high_bound_found: bool = binary_search(&nums, 10);
    let low_found_found: bool = binary_search(&nums, 1);
    let not_found: bool = binary_search(&nums, 20);

    println!(
        "found_true: {:?} \n high_bound_true: {:?} \n low_bound_true: {:?} \n not_found: {:?}",
        found_true, high_bound_found, low_found_found, not_found
    );
}

fn binary_search(nums: &[usize], target: usize) -> bool {
    let mut min: usize = 0;
    let mut max: usize = nums.len();

    while min < max {
        let mid: usize = (max - min) / 2 + min;
        let val: usize = nums[mid];

        if target == val {
            return true;
        }

        if val > target {
            // Check bottom half
            max = mid;
            continue;
        }

        min = mid + 1
    }

    false
}
