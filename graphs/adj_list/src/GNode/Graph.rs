#[derive(Debug)]
pub struct AdjacencyList {
    data: Vec<Vec<(usize, usize)>>,
}

impl Default for AdjacencyList {
    fn default() -> Self {
        Self {
            data: vec![
                vec![(1, 2)],
                vec![(2, 3)],
                vec![(3, 6)],
                vec![(4, 2)],
                vec![(5, 9)],
                vec![(0, 1)],
            ],
        }
    }
}

impl AdjacencyList {
    pub fn in_order_dfs(&self, target: usize, source: usize) -> Vec<i32> {
        if source > self.data.len() - 1 {
            return vec![];
        }

        let mut seen: Vec<bool> = vec![false; self.data.len()];
        let mut path = vec![-1; self.data.len()];

        self.iterate(&target, &source, &mut seen, &mut path, -1);
        self.create_path(&path, target)
    }

    pub fn iterate(
        &self,
        target: &usize,
        vertex: &usize,
        seen: &mut Vec<bool>,
        prnt_rel: &mut Vec<i32>,
        prev: i32,
    ) {
        seen[*vertex] = true;
        prnt_rel[*vertex] = prev;

        if vertex != target {
            for c in &self.data[*vertex] {
                let (next, _) = c;
                if !seen[*next] {
                    self.iterate(target, next, seen, prnt_rel, *vertex as i32);
                }
            }
        }
    }

    fn create_path(&self, path: &Vec<i32>, target: usize) -> Vec<i32> {
        if path[target] == -1 {
            vec![-1]
        } else {
            let mut final_path = vec![-1; path.len()];
            let mut new_target = path[target];
            let mut curr_idx = 0;

            while new_target > -1 {
                final_path[curr_idx] = new_target;
                curr_idx += 1;
                new_target = path[new_target as usize];
            }

            final_path.reverse();
            let trimmed: Vec<i32> = final_path.into_iter().filter(|&x| x > -1).collect();
            trimmed
        }
    }
}
