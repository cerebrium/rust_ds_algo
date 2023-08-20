fn main() {
    let mut nums: [usize; 19] = [
        0, 5, 6, 3, 4, 2, 3, 6, 8, 1, 90, 25, 34, 23, 64, 2, 3, 4, 87,
    ];
    quicksort(&mut nums);
    println!("Expecting sorted: {:?}", nums);
}

fn quicksort(nums: &mut [usize; 19]) {
    qs(nums, 0, nums.len() - 1);
}

fn qs(nums: &mut [usize; 19], low: usize, high: usize) {
    if low >= high {
        // All elements are at their most atomic
        return;
    }

    let pivot_idx = partition(nums, low, high);
    // Inclusive beggining and ending

    qs(nums, low, pivot_idx - 1);
    qs(nums, pivot_idx + 1, high);
}

fn partition(nums: &mut [usize; 19], low: usize, high: usize) -> usize {
    let pivot = nums[high];

    let mut idx: i32 = low as i32 - 1;

    for i in low..high {
        if nums[i] <= pivot {
            idx += 1;

            nums.swap(i, idx as usize);
        }
    }

    idx += 1;
    nums.swap(high, idx as usize);

    idx as usize
}
