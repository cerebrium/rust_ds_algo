use std::fmt::Display;

#[derive(Default)]
pub struct MinHeap {
    pub data: Vec<usize>,
}

impl MinHeap {
    pub fn add(&mut self, val: usize) {
        self.data.push(val);
        self.bubble_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<usize> {
        let length = self.data.len() - 1;

        if length < 1 {
            return None;
        }

        self.data.swap(0, length);
        let res = self.data.pop();

        MinHeap::bubble_down(self, 0);

        res
    }

    pub fn bubble_up(&mut self, idx: usize) {
        let mut curr_idx = idx;

        if self.data.len() < 2 {
            return;
        }

        loop {
            if curr_idx == 0 {
                return;
            }

            let parent_idx = MinHeap::get_parent_idx(curr_idx);

            if self.data[parent_idx] < self.data[curr_idx] {
                return;
            }

            self.data.swap(curr_idx, parent_idx);

            curr_idx = parent_idx;
        }
    }

    pub fn bubble_down(&mut self, idx: usize) {
        let (left_child_idx, right_child_idx) = MinHeap::get_both_children_idx(idx);

        // Base cases
        if idx >= self.data.len() - 1 || left_child_idx >= self.data.len() - 1 {
            return;
        }

        let curr_val = self.data[idx];
        let left_val = self.data[left_child_idx];
        let right_val = self.data[right_child_idx];

        if left_val > right_val && curr_val > right_val {
            self.data.swap(idx, right_child_idx);
            self.bubble_down(right_child_idx);
        } else if right_val > left_val && curr_val > left_val {
            self.data.swap(idx, left_child_idx);
            self.bubble_down(left_child_idx);
        }
    }

    fn get_parent_idx(idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn get_left_child_idx(idx: usize) -> usize {
        2 * idx + 1
    }

    fn get_right_child_idx(idx: usize) -> usize {
        2 * idx + 2
    }

    fn get_both_children_idx(idx: usize) -> (usize, usize) {
        (
            MinHeap::get_left_child_idx(idx),
            MinHeap::get_right_child_idx(idx),
        )
    }
}

impl Display for MinHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for num in &self.data {
            write!(f, "{}", num);
        }
        Ok(())
    }
}
