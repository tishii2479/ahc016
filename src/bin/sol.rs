#[allow(unused_imports)]
use std::fs::File;

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
    // const TIME_LIMIT: f64 = 4.8;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let flush = || stdout.lock().flush().unwrap();

    let (m, eps) = read_input(&stdin);

    let n = BEST_N[m - 10][(eps * 100.).round() as usize];

    eprintln!("M = {}, eps = {}, N = {}", m, eps, n);

    // M, epsに対応するグラフを出力する
    let graphs = create_optimal_graphs(n, m, eps, CONSTRUCT_TIME_LIMIT);

    // let mut log_file = File::create("data/visualizer.log").unwrap();

    println!("{}", n);
    // writeln!(log_file, "{} {}", n, m).unwrap();
    // writeln!(log_file, "{}", eps).unwrap();

    for graph in &graphs {
        println!("{}", graph.to_raw_format());
        // writeln!(log_file, "{}", raw_g).unwrap();
    }
    flush();

    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());

    // 各クエリを処理する
    for _ in 0..QUERY_COUNT {
        // eprintln!("Query: {}", q);
        let raw_h = read_graph_input(&stdin);
        // write!(log_file, "{}", raw_h).unwrap();
        // hとGとの類似度を求め、類似度が最大のGを出力する
        let h = Graph::from_raw_format(n, &raw_h);

        let best_graph_index = solve(&graphs, &h, eps);
        println!("{}", best_graph_index);
        // writeln!(log_file, "{}", best_graph_index).unwrap();
        flush();
        // eprintln!("selected: {}, dist: {}", best_graph_index, min_score);
    }

    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());
}
