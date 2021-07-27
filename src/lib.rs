
pub mod generator;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn generate_data() {
        let graph_decrease = crate::generator::generate_graph(
            5,
            10f64,
            -1f64,
            10
        ).await;

        let graph_increase = crate::generator::generate_graph(
            5,
            10f64,
            1f64,
            7
        ).await;

        println!("{:?}", graph_decrease.nodes.len());
        println!("{:?}", graph_increase.nodes.len());
    }
}