use crate::adjacency_matrix::adjacency_matrix::AdjM;

mod adjacency_matrix;

fn main() {
    let adj_m = AdjM::default();
    let path = adj_m.bfs(5);
    println!("Found path to 5: {:?}", path);
}
