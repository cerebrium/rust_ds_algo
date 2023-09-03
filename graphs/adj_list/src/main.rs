use GNode::Graph::AdjacencyList;

mod GNode;

fn main() {
    let graph = AdjacencyList::default();
    let path = graph.in_order_dfs(5, 0);
    println!("path: {:?}", path);
}
