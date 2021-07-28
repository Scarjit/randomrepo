pub mod generator;
pub mod edge;

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use std::path::Path;
    use std::fs;
    use tokio_postgres::{NoTls, Client};
    use std::borrow::Cow;
    use crate::edge::Edges;
    use std::fs::File;

    const SIMPLE_PATH: &str = concat!("C:/Users/ferdi/CLionProjects/randomrepo", "/sql/simple/");
    const V1_COP_PATH: &str =concat!("C:/Users/ferdi/CLionProjects/randomrepo", "/sql/v1/cop/");
    const V1_POC_PATH: &str = concat!("C:/Users/ferdi/CLionProjects/randomrepo", "/sql/v1/poc/");

    #[tokio::test]
    async fn generate_data() {
        let mut random = thread_rng();
        let mut all_nodes = 0;
        let mut all_edges = 0;
        let mut initial_id = 0;
        for i in 0..50 {
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

            let simple_path = format!("{}{}.sql", SIMPLE_PATH, i);
            let future_simple = graph.dump_sql_simple(Path::new(&simple_path));

            let v1_cop_path = format!("{}{}.sql", V1_COP_PATH, i);
            let v1_poc_path = format!("{}{}.sql", V1_POC_PATH, i);
            let future_split_v1 = graph.dump_sql_split_v1(
                Path::new(&v1_cop_path),
                Path::new(&v1_poc_path)
            );
            futures::join!(future_simple, future_split_v1);
            println!("\tFinished graph");
        }

        println!("---");
        println!("nNodes:\t{}", all_nodes);
        println!("nEdges:\t{}", all_edges);
    }


    #[tokio::test]
    async fn load_simple_data() {
        let client = get_conn().await;


//        let future_cop = insert_sql_data(V1_COP_PATH, &client);
//        let future_poc = insert_sql_data(V1_POC_PATH, &client);
        let future_simple = insert_sql_data(SIMPLE_PATH, &client);
        futures::join!(/*future_cop, future_poc, */future_simple);
    }

    async fn get_conn() -> Client {
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=postgres password=Katzen123 dbname=test", NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        client
    }


    #[tokio::test]
    async fn get_childen_of_zero_simple() {
        let client = get_conn().await;
        let sql = r"WITH RECURSIVE childen AS (
    SELECT
        parent,
        child
    FROM
        simple_table
    WHERE
        parent = $1
    UNION
        SELECT
            s.parent,
            s.child
        FROM
            simple_table s
        INNER JOIN childen c ON c.child = s.parent
)
SELECT *
FROM childen";

        let response = client.query(sql, &[&0]).await.unwrap();
        println!("nRow: {}", response.len());
        let mut rows = response.iter().map(|r|{
            (r.get::<usize, i32>(0) as usize,r.get::<usize, i32>(1) as usize)
        }).collect::<Vec<(usize, usize)>>();

        let mut f = File::create("simple.dot").unwrap();
        dot::render(&Edges(rows), &mut f).unwrap();
    }



    async fn insert_sql_data(path: &str, client: &Client){
        println!("{:?}", path);
        for f in fs::read_dir(path).unwrap() {
            let path = f.unwrap();
            let sql = async_fs::read_to_string(path.path()).await.unwrap();
            client.execute(sql.as_str(), &[]).await.unwrap();
        }
    }
}