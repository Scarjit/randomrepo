
pub mod generator;

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use std::path::Path;
    use std::fs;
    use tokio_postgres::NoTls;

    #[tokio::test]
    async fn generate_data() {
        let mut random = thread_rng();
        let mut all_nodes = 0;
        let mut all_edges = 0;
        let mut initial_id = 0;
        for i in 0..5 {
            let initial_children = random.gen_range(3..=5);
            let std_dev = random.gen_range(7f64..=10f64);
            let std_increase_factor = random.gen_range(-1f64..1f64);
            let mutator_percentage = random.gen_range(0..=10);
            println!("Generating graph with");
            println!("\tInitial children:\t\t{}", initial_children);
            println!("\tStd Dev:\t\t\t\t{}", std_dev);
            println!("\tStd Increase factor:\t{}", std_increase_factor);
            println!("\tMutator percentage:\t{}", std_increase_factor);

            let mut graph = crate::generator::Graph::generate(
                initial_children,
                std_dev,
                std_increase_factor,
                6,
                &mut initial_id
            ).await;
            graph.mutator(mutator_percentage).await;
            println!("\tNodes:\t\t\t\t\t{}", graph.nodes.len());
            println!("\tEdges:\t\t\t\t\t{}", graph.edges.len());
            all_nodes += graph.nodes.len();
            all_edges += graph.edges.len();
            graph.dump_sql_simple(Path::new(&format!("sql/simple/{}.sql", i))).await;
            println!("\tFinished graph");

        }

        println!("---");
        println!("nNodes:\t{}", all_nodes);
        println!("nEdges:\t{}", all_edges);
    }


    #[tokio::test]
    async fn load_simple_data() {
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=postgres password=Katzen123 dbname=test", NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });


        for f in fs::read_dir("sql/simple/").unwrap() {
            let path = f.unwrap();
            let sql = async_fs::read_to_string(path.path()).await.unwrap();
            client.execute(sql.as_str(), &[]).await.unwrap();
            println!("Executed {}", path.file_name().to_str().unwrap())
        }

    }
}