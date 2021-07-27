use rand_distr::{Distribution, Normal};
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter};
use async_fs::File;
use futures_lite::AsyncWriteExt;
use std::path::{Path};

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

impl Display for Graph{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nodes: {}\nEdges: {}", self.nodes.len(), self.edges.len())
    }
}

impl Graph{
    pub async fn generate(initial_childen: usize, std_dev: f64, std_increase_factor: f64, max_depth: usize, nodeid: &mut usize) -> Self{
        let mut graph: Graph = Graph{
            nodes: vec![],
            edges: vec![]
        };

        for _ in 0..initial_childen {
            graph.nodes.push(Node{std_dev, id: *nodeid });
            *nodeid += 1;
        }

        let mut current_std_dev = std_dev;
        let mut depth = 0;
        while current_std_dev > 0f64 && depth < max_depth {
            let new_std_dev = current_std_dev + std_increase_factor;
            for node in graph.nodes.clone() {
                if node.std_dev == current_std_dev {
                    let childs = Self::generate_childen(node.std_dev).await;
                    for _ in 0..childs {
                        graph.nodes.push(Node{std_dev: new_std_dev, id: *nodeid });
                        graph.edges.push( (node.id, *nodeid));
                        *nodeid += 1;
                    }
                }
            }
            current_std_dev = new_std_dev;
            depth += 1;
        }
        return graph
    }

    async fn generate_childen(std_dev: f64) -> usize {
        let mut rng = thread_rng();
        let normal = Normal::new(0f64, std_dev).unwrap();
        normal.sample(&mut rng).abs().round() as usize
    }


    pub async fn mutator(self: &mut Graph, mutation_percent: usize){
        let max_nodes = &self.nodes.len();
        let n_picked_nodes = (*max_nodes as f64 * (mutation_percent as f64/100f64)).round();
        let mut rng = thread_rng();
        for choosen_node in self.nodes.choose_multiple(&mut rng, n_picked_nodes as usize) {
            let other_node = self.nodes.choose(&mut rng).unwrap();
            self.edges.push((choosen_node.id, other_node.id))
        }
    }

    pub async fn dump_sql_simple(self: Graph, filename: &Path){
        let mut f = File::create(filename).await.unwrap();

        let mut sql = String::from("INSERT INTO simple_table (parent, child) VALUES\n");

        for edge in self.edges {
            sql += &format!("\t({}, {}),\n", edge.0, edge.1);
        }
        sql.remove(sql.len()-1);
        sql.remove(sql.len()-1);
        sql += ";";

        f.write_all(sql.as_bytes()).await.unwrap();
    }

}



