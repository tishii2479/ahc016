use crate::{
    gen::State,
    graph::{calc_graph_similarity, operate_toggle, Graph},
};

pub fn solve(state: &State, h: &Graph, eps: f64) -> usize {
    // let h_edge_count = h.calc_edge_count();

    // TODO: epsの考慮
    // let min_edge_count = i64::max(0, h_edge_count as i64 - 100) as usize;
    // let max_edge_count = h_edge_count + 100;

    let mut best_graph_index = 0;
    let mut min_score = i64::MAX;

    // TODO: 類似度を求めるのを賢くする
    for i in 0..state.graphs.len() {
        // let graph_edge_count = state.graphs[i].calc_edge_count();
        // let expected_graph_edge_count = (graph_edge_count as f64 * (1. - 2. * eps)
        //     + (n * (n - 1) / 2) as f64 * eps)
        //     as usize;
        // if expected_graph_edge_count < min_edge_count
        //     || max_edge_count < expected_graph_edge_count
        // {
        //     continue;
        // }

        // モンテカルロ法
        // `TRIAL_COUNT`回ランダムにグラフに操作をして、その結果のグラフとhの類似度の平均を使う
        const TRIAL_COUNT: usize = 500;
        let mut score_sum = 0;
        for _ in 0..TRIAL_COUNT {
            let mut graph = state.graphs[i].clone();
            operate_toggle(&mut graph, eps);
            score_sum += calc_graph_similarity(&h, &graph);
        }
        if score_sum < min_score {
            min_score = score_sum;
            best_graph_index = i;
        }
    }

    best_graph_index
}
