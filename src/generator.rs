use rand_distr::{Distribution, Normal, NormalError};
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Node {
    pub std_dev: f64,
    pub id: usize
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<(usize, usize)>
}

pub async fn generate_graph(initial_childen: usize, std_dev: f64, std_increase_factor: f64, max_depth: usize) -> Graph {
    let mut graph: Graph = Graph{
        nodes: vec![],
        edges: vec![]
    };

    let mut nodeid = 0;
    for i in 0..initial_childen {
        graph.nodes.push(Node{std_dev, id: nodeid });
        nodeid += 1;
    }

    let mut current_std_dev = std_dev;
    let mut depth = 0;
    while current_std_dev > 0f64 && depth < max_depth {
        let new_std_dev = current_std_dev + std_increase_factor;
        for node in graph.nodes.clone() {
            if node.std_dev == current_std_dev {
                let childs = generate_childen(node.std_dev).await;
                for i in 0..childs {
                    graph.nodes.push(Node{std_dev: new_std_dev, id: nodeid });
                    graph.edges.push((node.id, nodeid));
                    nodeid += 1;
                }
            }
        }
        current_std_dev = new_std_dev;
        depth += 1;
    }
    return graph
}


pub async fn generate_childen(std_dev: f64) -> usize {
    let mut rng = thread_rng();
    let normal = Normal::new(0f64, std_dev).unwrap();
    normal.sample(&mut rng).abs().round() as usize
}

pub async fn graph_mutator(graph: &mut Graph, mutation_percent: usize){
    let max_nodes = &graph.nodes.len();
    let n_picked_nodes = max_nodes as f64 * (mutation_percent as f64/100f64);

}