use std::{cell::Cell, collections::HashMap};

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
        visited: Cell::new(HashMap::new()),
    };
    let path = new_map.find_path();
    println!("Path: {:?}", path);
}

struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    visited: Cell<HashMap<String, bool>>,
}

impl Map {
    pub fn find_path(&mut self) -> Vec<(usize, usize)> {
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

        let answer = for x in 0..height {
            match x {
                0 => {
                    for y in 0..self.map[x].len() {
                        let val = self.map[x][y];
                        if val == 'S' {
                            if let Some(path) = Self::_iterate(&self, (x, y), vec![]) {
                                return path;
                            }
                        }
                    }
                }
                height => {
                    for y in 0..self.map[x].len() {
                        let val = self.map[x][y];
                        if val == 'S' {
                            if let Some(path) = Self::_iterate(&self, (x, y), vec![]) {
                                println!("what is the path: {:?}", path);
                                return path;
                            }
                        }
                    }
                }
                _ => {
                    // First and last indices
                    let first = self.map[x][0];
                    let last = self.map[x][width];

                    if first == 'S' {
                        if let Some(path) = Self::_iterate(&self, (x, 0), vec![]) {
                            return path;
                        }
                    }
                    if last == 'S' {
                        if let Some(path) = Self::_iterate(&self, (x, width), vec![]) {
                            return path;
                        }
                    }
                }
            }
        };
        println!("answer: {:?}", answer);
        vec![]
    }

    fn _iterate(
        &self,
        (x, y): (usize, usize),
        mut path: Vec<(usize, usize)>,
    ) -> Option<Vec<(usize, usize)>> {
        println!("terate running: {:?}", (x, y));

        let mut str_x_y = y.to_string();
        let x_str = x.to_string();
        str_x_y.push_str(&x_str);

        self.visited.take().insert(str_x_y, true);

        if let Some(arr) = self.map.get(x) {
            println!("what is the arr: {:?}", arr);
            if let Some(val) = arr.get(y) {
                println!("what is the val: {:?}", val);
                match val {
                    'P' | 'S' => {
                        path.push((x, y));

                        println!("inside the ");
                        for viable_target in Self::viable_paths(&self, (x, y)) {
                            let res = Self::_iterate(&self, viable_target, path.clone());
                            if let Some(end) = res {
                                return Some(end);
                            }
                        }
                    }
                    'E' => {
                        println!("inside the end");
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
                let x_p_o = x + 1;
                let mut str_x_y = x_p_o.to_string();
                let y_str = y.to_string();
                str_x_y.push_str(&y_str);

                str_x_y.push('y');
                if !self.visited.take().contains_key(&str_x_y) {
                    viable_paths.push((x + 1, y))
                }
            })
        });

        self.map.get(x - 1).map(|arr| {
            arr.get(y).map(|_| {
                let x_p_o = x - 1;
                let mut str_x_y = x_p_o.to_string();
                let y_str = y.to_string();
                str_x_y.push_str(&y_str);

                if !self.visited.take().contains_key(&str_x_y) {
                    viable_paths.push((x - 1, y))
                }
            })
        });

        self.map.get(x).map(|arr| {
            arr.get(y + 1).map(|_| {
                let y_p_o = y + 1;
                let mut str_x_y = y_p_o.to_string();
                let x_str = x.to_string();
                str_x_y.push_str(&x_str);

                if !self.visited.take().contains_key(&str_x_y) {
                    viable_paths.push((x, y + 1))
                }
            })
        });

        self.map.get(x).map(|arr| {
            arr.get(y - 1).map(|_| {
                let y_p_o = y - 1;
                let mut str_x_y = y_p_o.to_string();
                let x_str = x.to_string();
                str_x_y.push_str(&x_str);

                if !self.visited.take().contains_key(&str_x_y) {
                    viable_paths.push((x, y - 1))
                }
            })
        });

        viable_paths
    }
}
