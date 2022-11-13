use crate::{
    gen::State,
    graph::{calc_graph_similarity, operate_toggle, Graph},
    util::time,
};

pub fn solve(state: &State, h: &Graph, eps: f64, time_limit: f64) -> usize {
    let start_time = time::elapsed_seconds();

    const E: f64 = 5.;
    let n = h.n;
    let h_edge_count = h.calc_edge_count();
    let max_edge_count = n * (n - 1) / 2;
    let error_width = (max_edge_count as f64 * eps * (1. - eps)).powf(0.5) * E;

    let min_edge_count = i64::max(0, h_edge_count as i64 - error_width as i64) as usize;
    let max_edge_count = h_edge_count + error_width as usize;

    let mut best_graph_index = 0;
    let mut min_score = 1e10;

    let mut candidate_graph_count: usize = 0;

    let expected_graph_edge_count = |edge_count: usize| {
        (edge_count as f64 * (1. - 2. * eps) + max_edge_count as f64 * eps) as usize
    };

    for i in 0..state.graphs.len() {
        let graph_edge_count = state.graphs[i].calc_edge_count();
        let expected_graph_edge_count = expected_graph_edge_count(graph_edge_count);
        if min_edge_count <= expected_graph_edge_count
            && expected_graph_edge_count <= max_edge_count
        {
            candidate_graph_count += 1;
        }
    }

    let has_candidate = candidate_graph_count > 0;
    if !has_candidate {
        candidate_graph_count = state.graphs.len();
    }

    for i in 0..state.graphs.len() {
        let graph_edge_count = state.graphs[i].calc_edge_count();
        let expected_graph_edge_count = expected_graph_edge_count(graph_edge_count);
        if has_candidate
            && (expected_graph_edge_count < min_edge_count
                || max_edge_count < expected_graph_edge_count)
        {
            continue;
        }

        // モンテカルロ法
        // `TRIAL_COUNT`回ランダムにグラフに操作をして、その結果のグラフとhの類似度の平均を使う
        let current_time = time::elapsed_seconds();
        let usable_time = (start_time + time_limit - current_time) / candidate_graph_count as f64;

        let mut counter = 0;
        let mut score_sum = 0;

        // TODO: 時間管理を効率的に
        while time::elapsed_seconds() - current_time < usable_time {
            let mut graph = state.graphs[i].clone();
            operate_toggle(&mut graph, eps);
            score_sum += calc_graph_similarity(&h, &graph);
            counter += 1;
        }

        let score = score_sum as f64 / counter as f64;
        if score < min_score {
            min_score = score;
            best_graph_index = i;
        }
        candidate_graph_count -= 1;
    }

    best_graph_index
}
