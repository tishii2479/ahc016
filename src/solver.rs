use std::ops::RangeInclusive;

use crate::graph::{calc_graph_similarity, Graph};

pub fn solve(graphs: &Vec<Graph>, h: &Graph, m: usize, eps: f64) -> usize {
    let n = h.n;
    let h_edge_count = h.calc_edge_count();
    let max_edge_count = n * (n - 1) / 2;

    let mut candidate_graph_count: usize = 0;

    let expected_graph_edge_count = |edge_count: usize| {
        (edge_count as f64 * (1. - 2. * eps) + max_edge_count as f64 * eps) as usize
    };

    let mut err = 4.;
    let mut edge_count_range: RangeInclusive<usize>;

    // 辺の総数の期待値が標準偏差 * err の範囲にあるグラフの数を数えて、
    // その数が0以上になるまで err を増やす
    // 調べるグラフの数を削減することが目的
    // TODO: 必要かどうかチェック
    loop {
        let std_var = (max_edge_count as f64 * eps * (1. - eps)).powf(0.5);
        let error_width = std_var * err;

        let min_edge_count = i64::max(0, h_edge_count as i64 - error_width as i64) as usize;
        let max_edge_count = h_edge_count + error_width as usize;

        edge_count_range = min_edge_count..=max_edge_count;

        for i in 0..graphs.len() {
            let is_occurable =
                edge_count_range.contains(&expected_graph_edge_count(graphs[i].calc_edge_count()));
            if is_occurable {
                candidate_graph_count += 1;
            }
        }
        // error_widthの範囲を広げる
        err += 1.;
        if candidate_graph_count > 0 {
            break;
        }
    }

    let mut best_graph_index = 0;
    let mut min_score = 1e10;

    for i in 0..graphs.len() {
        let score = calc_graph_similarity(&h, &graphs[i], m, eps);
        if score < min_score {
            min_score = score;
            best_graph_index = i;
        }

        candidate_graph_count -= 1;
    }

    best_graph_index
}
