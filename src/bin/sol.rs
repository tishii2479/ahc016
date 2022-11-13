use std::io;
use std::io::Write;

use ahc016::gen_graph::*;
use ahc016::graph::*;
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
    let n = if eps == 0. { 10 } else { m };

    let state = create_optimal_graphs(n, m, 10000);
    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());

    println!("{}", n);
    state.output();
    flush();

    // 各クエリを処理する
    for q in 0..QUERY_COUNT {
        eprintln!("Query: {}", q);

        let h = read_graph_input(&stdin);
        let h_edge_count = h.matches("1").count();

        // TODO: epsの考慮
        let min_edge_count = i64::max(0, h_edge_count as i64 - 100) as usize;
        let max_edge_count = h_edge_count + 100;

        // hとGとの類似度を求め、類似度が最大のGを出力する
        let h = Graph::from_raw_format(n, &h);

        let mut min_graph_index = 0;
        let mut min_dist = i64::MAX;

        // 類似度を求めるのを賢くする
        for i in 0..m {
            let graph_edge_count = state.graphs[i].calc_edge_count();
            if graph_edge_count < min_edge_count || max_edge_count < graph_edge_count {
                continue;
            }
            // let dist = calc_graph_similarity_with_hill_climbing(
            //     &h,
            //     &state.graphs[i],
            //     100000,
            //     50.,
            //     1.,
            //     false,
            // );
            let dist = calc_graph_similarity(&h, &state.graphs[i]);
            if dist < min_dist {
                min_dist = dist;
                min_graph_index = i;
            }
        }

        println!("{}", min_graph_index);
        eprintln!("selected: {}, dist: {}", min_graph_index, min_dist);
        flush();
    }

    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());
}
