use std::io;
use std::io::Write;

use ahc016::gen_graph::*;
use ahc016::graph::*;

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
    const QUERY_COUNT: usize = 100;
    const ITER_COUNT: usize = 200;

    let stdin = io::stdin();
    let stdout = io::stdout();

    let (m, eps) = read_input(&stdin);

    // M, epsに対応するグラフを出力する
    let n = if eps == 0. { 10 } else { m };

    let state = create_optimal_graphs(n, m, ITER_COUNT);

    println!("{}", n);
    stdout.lock().flush().unwrap();

    state.output();

    // 各クエリを処理する
    for _ in 0..QUERY_COUNT {
        let h = read_graph_input(&stdin);

        // hとGとの類似度を求め、類似度が最大のGを出力する
        let h = Graph::from_raw_format(n, &h);
        let mut min_graph_index = 0;
        let mut min_dist = i64::MAX;

        // 類似度を求めるのを賢くする
        for i in 0..m {
            let dist = calc_graph_similarity(&h, &state.graphs[i]);
            if dist < min_dist {
                min_dist = dist;
                min_graph_index = i;
            }
        }

        println!("{}", min_graph_index);
        stdout.lock().flush().unwrap();
    }
}
