use crate::graph::{calc_graph_similarity, Graph};

pub fn solve(graphs: &Vec<Graph>, h: &Graph, m: usize, eps: f64) -> usize {
    let mut best_graph_index = 0;
    let mut min_score = 1e10;

    for i in 0..graphs.len() {
        let score = calc_graph_similarity(&h, &graphs[i], m, eps);
        if score < min_score {
            min_score = score;
            best_graph_index = i;
        }
    }

    best_graph_index
}
