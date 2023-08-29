use crate::adjacency_matrix::adjacency_matrix::AdjM;

mod adjacency_matrix;

fn main() {
    let adj_m = AdjM::default();
    let path = adj_m.bfs(4);
    println!("Found path to 4: {:?}", path);
}
