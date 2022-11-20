use std::io;
use std::io::Write;

use ahc016::gen::*;
use ahc016::graph::*;
use ahc016::param::BEST_N;
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
    const CONSTRUCT_TIME_LIMIT: f64 = 4.7;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let flush = || stdout.lock().flush().unwrap();

    let (m, eps) = read_input(&stdin);

    let n = BEST_N[m - 10][(eps * 100.).round() as usize];

    eprintln!("M = {}, eps = {}, N = {}", m, eps, n);

    // M, epsに対応するグラフを出力する
    let graphs = create_optimal_graphs(n, m, eps, CONSTRUCT_TIME_LIMIT);

    println!("{}", n);

    for graph in &graphs {
        println!("{}", graph.to_raw_format());
    }
    flush();

    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());

    // 各クエリを処理する
    for _ in 0..QUERY_COUNT {
        let raw_h = read_graph_input(&stdin);

        // hとGとの類似度を求め、類似度が最大のGを出力する
        let mut h = Graph::from_raw_format(n, &raw_h);
        // h.degreesをシミュレーション後の次数列とみなす
        h.simulated_degrees = h.degrees.clone();
        h.simulated_degrees
            .sort_by(|a, b| a.partial_cmp(b).unwrap());
        h.simulated_squares = calc_simulated_square(&h);

        let best_graph_index = solve(&graphs, &h, m, eps);
        println!("{}", best_graph_index);
        flush();
    }

    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());
}
