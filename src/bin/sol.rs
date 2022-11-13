use std::io;
use std::io::Write;

use ahc016::gen_graph::*;
use ahc016::graph::*;
use ahc016::util;
use ahc016::util::time;

fn read_input(stdin: &io::Stdin) -> (usize, f64) {
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
    let mut v = vec![];
    for e in user_input.trim().split(" ") {
        v.push(e.to_string());
    }
    (v[0].parse().unwrap(), v[1].parse().unwrap())
}

fn read_graph_input(stdin: &io::Stdin) -> String {
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
    user_input
}

fn main() {
    time::start_clock();
    const QUERY_COUNT: usize = 100;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let flush = || stdout.lock().flush().unwrap();

    let (m, eps) = read_input(&stdin);

    // M, epsに対応するグラフを出力する
    let n = if eps == 0. { 10 } else { 20 };

    let state = create_optimal_graphs(n, m, 10000);
    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());

    println!("{}", n);
    state.output();
    flush();

    // 各クエリを処理する
    for q in 0..QUERY_COUNT {
        eprintln!("Query: {}", q);

        let h = read_graph_input(&stdin);
        // let h_edge_count = h.matches("1").count();

        // TODO: epsの考慮
        // let min_edge_count = i64::max(0, h_edge_count as i64 - 100) as usize;
        // let max_edge_count = h_edge_count + 100;

        // hとGとの類似度を求め、類似度が最大のGを出力する
        let h = Graph::from_raw_format(n, &h);

        let mut best_graph_index = 0;
        let mut min_score = i64::MAX;

        // TODO: 類似度を求めるのを賢くする
        for i in 0..m {
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
            // trial回ランダムにグラフに操作をして、その結果のグラフとhの類似度の平均を使う
            const TRIAL_COUNT: usize = 500;
            let mut score_sum = 0;
            for _ in 0..TRIAL_COUNT {
                let mut graph = state.graphs[i].clone();
                // TODO: maskに変えて、xorをとる
                for j in 0..graph.get_edge_size() {
                    if util::rnd::nextf() < eps {
                        graph.toggle_edge(j);
                    }
                }
                score_sum += calc_graph_similarity(&h, &graph);
            }
            if score_sum < min_score {
                min_score = score_sum;
                best_graph_index = i;
            }
        }

        println!("{}", best_graph_index);
        eprintln!("selected: {}, dist: {}", best_graph_index, min_score);
        flush();
    }

    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());
}
