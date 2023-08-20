use std::collections::HashSet;

fn main() {
    let test_data: Vec<Vec<char>> = vec![
        vec!['W', 'W', 'W', 'w', 'E', 'W'],
        vec!['W', 'P', 'P', 'P', 'P', 'P'],
        vec!['W', 'S', 'W', 'W', 'W', 'W'],
    ];
    let height = test_data.len();
    let width = test_data[0].len();

    let mut new_map = Map {
        map: test_data,
        height,
        width,
        visited: HashSet::new(),
    };
    let path = new_map.find_path();
    if let Some(final_path) = path {
        println!(" {:?}", final_path);
    }
}

struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    visited: HashSet<usize>,
}

impl Map {
    pub fn find_path(&mut self) -> Option<Vec<(usize, usize)>> {
        /*
         *
         * Base Cases:
         * 1. If 'E' end: true
         * 2. if 'W' wall: false
         * 3. Off the map: false
         * 4. Already visited: false;
         *
         */
        let height = self.height;
        let width = self.width - 1;

        for x in 0..height {
            match x {
                0 => {
                    for y in 0..self.map[x].len() {
                        let val = self.map[x][y];
                        if val == 'S' {
                            return Self::_iterate(self, (x, y), vec![]);
                        }
                    }
                }
                height => {
                    for y in 0..self.map[x].len() {
                        let val = self.map[x][y];
                        if val == 'S' {
                            return Self::_iterate(self, (x, y), vec![]);
                        }
                    }
                }
                _ => {
                    // First and last indices
                    let first = self.map[x][0];
                    let last = self.map[x][width];

                    if first == 'S' {
                        return Self::_iterate(self, (x, 0), vec![]);
                    }
                    if last == 'S' {
                        return Self::_iterate(self, (x, width), vec![]);
                    }
                }
            };
        }
        None
    }

    fn _iterate(
        &mut self,
        (x, y): (usize, usize),
        mut path: Vec<(usize, usize)>,
    ) -> Option<Vec<(usize, usize)>> {
        let now_visited = self.map[0].len() * x + y;

        self.visited.insert(now_visited);

        if let Some(arr) = &mut self.map.get(x) {
            if let Some(val) = arr.get(y) {
                match val {
                    'P' | 'S' => {
                        path.push((x, y));

                        for viable_target in Self::viable_paths(self, (x, y)) {
                            let res = Self::_iterate(self, viable_target, path.clone());
                            if let Some(end) = res {
                                return Some(end);
                            }
                        }
                    }
                    'E' => {
                        path.push((x, y));
                        return Some(path);
                    }
                    _ => return None,
                };
            };
        };

        None
    }

    fn viable_paths(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut viable_paths = vec![];

        self.map.get(x + 1).map(|arr| {
            arr.get(y).map(|_| {
                let now_visited = self.map[0].len() * (x + 1) + y;

                if !&self.visited.contains(&now_visited) {
                    viable_paths.push((x + 1, y))
                }
            })
        });

        self.map.get(x - 1).map(|arr| {
            arr.get(y).map(|_| {
                let now_visited = self.map[0].len() * (x - 1) + y;

                if !&self.visited.contains(&now_visited) {
                    viable_paths.push((x - 1, y))
                }
            })
        });

        self.map.get(x).map(|arr| {
            arr.get(y + 1).map(|_| {
                let now_visited = self.map[0].len() * x + (y + 1);

                if !&self.visited.contains(&now_visited) {
                    viable_paths.push((x, y + 1))
                }
            })
        });

        self.map.get(x).map(|arr| {
            arr.get(y - 1).map(|_| {
                let now_visited = self.map[0].len() * x + (y - 1);

                if !&self.visited.contains(&now_visited) {
                    viable_paths.push((x, y - 1))
                }
            })
        });

        viable_paths
    }
}
