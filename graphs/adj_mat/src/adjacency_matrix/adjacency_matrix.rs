#[derive(Debug)]
pub struct AdjM {
    data: Vec<Vec<usize>>,
}

impl AdjM {
    pub fn bfs(&self, target: usize) -> Vec<i32> {
        if target > self.data.len() {
            return vec![-1];
        }
        let mut q: Vec<usize> = vec![0];
        let mut v = vec![0; self.data.len()];
        let mut p = vec![-1];
        let mut prev_idx = 0;

        for _ in 0..self.data.len() - 1 {
            p.push(-1);
        }

        v[0] = 1;

        loop {
            if q.is_empty() {
                break;
            }

            let c = q.pop();
            if let Some(val) = c {
                if val == target {
                    break;
                }

                /*
                 *
                 * Visited is corresponding to the index.
                 * The index of the sub vector is the vertex,
                 * if visited, than the vertex index in the
                 * visited vector becomes true.
                 *
                 * The previous vector also is index based. When
                 * parsing, we will look to the previous vector
                 * to know as index (i) what is the origin (p[i]).
                 *
                 */
                for vertex in 0..self.data[val].len() {
                    // Child is weight of the edge
                    let weight = &self.data[val][vertex];
                    if *weight == 0 {
                        continue;
                    }

                    let o_visited = v.get(vertex);
                    if let Some(is_visited) = o_visited {
                        if *is_visited == 1 {
                            continue;
                        }
                    }

                    p[val] = prev_idx;
                    prev_idx = val as i32;
                    v[vertex] = 1;
                    q.push(vertex);
                }
            }
        }

        /*
         *
         * If idx in previous vector is not -1, than it is
         * found. Begin with target. The value at p[target]
         * is the node that has target as a child. Walk back
         * until -1, to find the path.
         */
        if prev_idx == -1 {
            vec![-1]
        } else {
            let mut next = prev_idx as usize;
            let mut prev_next = p.len() + 1;
            let mut path: Vec<i32> = vec![];

            /*
             *
             * v arr:
             * idx: vertex
             * val: parent
             *
             */

            loop {
                if next > p.len() || p[next] < 0 || prev_next == next {
                    break;
                }

                path.push(next as i32);
                prev_next = next;
                next = p[next] as usize;
            }

            path.reverse();
            path
        }
    }
}

// Should be a circle
impl Default for AdjM {
    fn default() -> Self {
        Self {
            data: vec![
                vec![0, 2, 0, 0, 0, 0],
                vec![0, 0, 2, 0, 0, 0],
                vec![0, 0, 0, 2, 0, 0],
                vec![0, 0, 0, 0, 2, 0],
                vec![0, 0, 0, 0, 0, 2],
                vec![2, 0, 0, 0, 0, 0],
            ],
        }
    }
}
