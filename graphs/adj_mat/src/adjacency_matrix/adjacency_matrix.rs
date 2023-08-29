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

                    p[*weight] = val as i32;
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
        if p[target] == -1 {
            vec![-1]
        } else {
            let mut next = target;
            let mut path: Vec<i32> = vec![];
            while let Some(decendant) = p.get(next) {
                if *decendant == -1 {
                    break;
                }
                path.push(*decendant);
                next = path[*decendant as usize] as usize;
                println!("inside the while: {:?}", next);
            }

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
