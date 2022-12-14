use std::env;

#[allow(unused_imports)]
use ahc016::{
    gen::{create_optimal_graphs, create_optimal_graphs_greedy},
    graph::{calc_simulated_square, operate_toggle},
    solver::solve,
};

// 各M, epsについて、最適なNを探し、そのグラフを出力する
fn main() {
    const TEST_COUNT: usize = 100;
    const CONSTRUCT_TIME_LIMIT: f64 = 2.8;
    const TRIAL_COUNT: usize = 5;

    let args: Vec<String> = env::args().collect();
    let m = args[1].parse::<usize>().unwrap();
    let eps = args[2].parse::<f64>().unwrap();
    let n = args[3].parse::<usize>().unwrap();

    eprintln!("{}, {}, {}", m, eps, n);

    let mut counts = vec![];

    for _ in 0..TRIAL_COUNT {
        let mut correct_count = 0;
        let graphs = create_optimal_graphs(n, m, eps, CONSTRUCT_TIME_LIMIT);
        for i in 0..m {
            let answer_graph_index = i;
            let mut h = graphs[answer_graph_index].clone();

            operate_toggle(&mut h, eps);
            h.simulated_degrees = h.degrees.clone();
            h.simulated_degrees
                .sort_by(|a, b| a.partial_cmp(b).unwrap());
            h.simulated_squares = calc_simulated_square(&h);

            let expected_graph_index = solve(&graphs, &h, m, eps);

            if answer_graph_index == expected_graph_index {
                correct_count += 1;
            }
        }
        eprintln!("correct_count: {}", correct_count);

        counts.push(correct_count);
    }

    let cut = 1;
    let all_trial_count = (TRIAL_COUNT - cut * 2) * m;
    let mut correct_count = 0;
    counts.sort();
    for i in cut..(TRIAL_COUNT - cut) {
        correct_count += counts[i];
    }
    let wrong_count = all_trial_count - correct_count;
    let score = 1e9 * 0.9_f64.powf(wrong_count as f64 / all_trial_count as f64 * TEST_COUNT as f64)
        / n as f64;
    eprintln!("Result = {},{},{},{}", m, eps, n, score);
    println!("Result = {},{},{},{}", m, eps, n, score);
}
