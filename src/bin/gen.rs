use std::ops::RangeInclusive;

use ahc016::{
    graph::{self, calc_graph_similarity, Graph},
    util::rnd,
};
use proconio::input;

fn create_initial_graphs(n: usize, m: usize) -> Vec<Graph> {
    let mut graphs = vec![];
    let max_graph_size = n * (n - 1) / 2;
    for i in 0..m {
        let mut graph_raw_format = vec![false; max_graph_size];
        let graph_size = max_graph_size * i / (m - 1);
        for _ in 0..graph_size {
            let mut i: usize = rnd::gen_range(0, max_graph_size);
            while graph_raw_format[i] == true {
                i = rnd::gen_range(0, max_graph_size);
            }
            graph_raw_format[i] = true;
        }
        graphs.push(Graph::from_vec_format(n, graph_raw_format));
    }
    return graphs;
}

fn create_optimal_graphs(n: usize, m: usize, iter_count: usize) -> State {
    // M個のグラフを初期化する
    let graphs = create_initial_graphs(n, m);
    let mut state = State::new(graphs);

    // 焼きなまし
    for iter in 0..iter_count {
        let current_score = state.score;

        // 辺を付け替える
        let mut graph_index;
        let mut edge_index1;
        let mut edge_index2;

        loop {
            graph_index = rnd::gen_range(0, m);
            edge_index1 = rnd::gen_range(0, n * (n - 1) / 2);
            edge_index2 = rnd::gen_range(0, n * (n - 1) / 2);

            if state.graphs[graph_index].has_edge(edge_index1)
                != state.graphs[graph_index].has_edge(edge_index2)
            {
                state.graphs[graph_index].toggle_edge(edge_index1);
                state.graphs[graph_index].toggle_edge(edge_index2);
                break;
            }
        }

        // グラフの距離を計算する
        let new_score = state.calc_score();

        if new_score > current_score {
            // 採用
            state.score = new_score;
        } else {
            // 不採用、ロールバック
            state.graphs[graph_index].toggle_edge(edge_index1);
            state.graphs[graph_index].toggle_edge(edge_index2);
        }
    }

    state
}

#[derive(Debug)]
struct State {
    score: i64,
    graphs: Vec<Graph>,
}

impl State {
    fn new(graphs: Vec<Graph>) -> State {
        let mut state = State { score: 0, graphs };
        state.score = state.calc_score();
        state
    }

    fn calc_score(&self) -> i64 {
        // 各グラフについて、最も近いグラフとの距離の総和
        // 大きいほどよい
        // TODO: 全てのグラフと試すのではなく、何個かサンプリングする
        // TODO: 更新したグラフのみ差分計算をする
        let mut score = 0;
        for i in 0..self.graphs.len() {
            let mut min_dist = i64::MAX;
            for j in 0..self.graphs.len() {
                if i == j {
                    continue;
                }
                min_dist = i64::min(
                    min_dist,
                    calc_graph_similarity(&self.graphs[i], &self.graphs[j]),
                );
            }
            score += min_dist;
        }
        score
    }
}

fn main() {
    const N_RANGE: RangeInclusive<usize> = 10..=10;
    const ITER_COUNT: usize = 10000;

    input! {
        m: usize,
        eps: f64
    }

    for n in N_RANGE {
        // グラフを作成して出力する
        let state = create_optimal_graphs(n, m, ITER_COUNT);
        eprintln!("{:?}", state);
    }
}
