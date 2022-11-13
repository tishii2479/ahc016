use std::io;
use std::io::Write;

use ahc016::gen::*;
use ahc016::graph::*;
use ahc016::solver::solve;
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
    const CONSTRUCT_TIME_LIMIT: f64 = 1.5;
    const TIME_LIMIT: f64 = 4.8;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let flush = || stdout.lock().flush().unwrap();

    let (m, eps) = read_input(&stdin);

    // TODO: 最適なNを埋め込む
    let n = if eps == 0. { 10 } else { 30 };

    // M, epsに対応するグラフを出力する
    let state = create_optimal_graphs(n, m, eps, CONSTRUCT_TIME_LIMIT);
    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());

    println!("{}", n);
    let g = state.format_to_string();
    for raw_g in g.split(" ") {
        println!("{}", raw_g);
    }
    flush();

    // 各クエリを処理する
    for q in 0..QUERY_COUNT {
        let remaining_time = TIME_LIMIT - time::elapsed_seconds();
        let time_limit: f64 = remaining_time / (QUERY_COUNT - q) as f64;
        // eprintln!("Query: {}", q);
        let raw_h = read_graph_input(&stdin);
        // hとGとの類似度を求め、類似度が最大のGを出力する
        let h = Graph::from_raw_format(n, &raw_h);

        let best_graph_index = solve(&state, &h, eps, time_limit);
        println!("{}", best_graph_index);
        flush();

        // eprintln!("selected: {}, dist: {}", best_graph_index, min_score);
    }

    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());
}
