use std::{env, fs::File, io::Read, ops::RangeInclusive};

use ahc016::{gen::State, graph::operate_toggle, solver::solve, util};

// 各M, epsについて、最適なNを探し、そのグラフを出力する
fn main() {
    // WARN: 正しくは 4..=100、一時的にNの数を小さくしている
    const N_RANGE: RangeInclusive<usize> = 10..=100;

    const TEST_COUNT: usize = 5;
    const TIME_LIMIT: f64 = 3.;

    let args: Vec<String> = env::args().collect();
    let m = args[1].parse::<usize>().unwrap();
    let eps = args[2].parse::<f64>().unwrap();

    let mut best_n = 0;
    let mut best_score = 0.;
    let e = ((eps * 100.) as i64).to_string();

    // WARN: 正しくは 4..=100、一時的にNの数を小さくしている
    for n in N_RANGE.step_by(10) {
        let file_path = format!("data/graphs/{m}_{e}/{n}.txt");

        if let Ok(mut file) = File::open(&file_path) {
            // グラフを読み込んで解き、正解率を出力する
            let mut raw_data = String::new();
            file.read_to_string(&mut raw_data).unwrap();

            let state = State::from_raw_format(n, &raw_data);

            let mut correct_count = 0;

            for _ in 0..TEST_COUNT {
                let answer_graph_index = util::rnd::gen_range(0, m);
                let mut h = state.graphs[answer_graph_index].clone();

                operate_toggle(&mut h, eps);
                let expected_graph_index = solve(&state, &h, eps, TIME_LIMIT);

                if answer_graph_index == expected_graph_index {
                    correct_count += 1;
                }
            }

            let wrong_count = TEST_COUNT - correct_count;
            let score = 1e9 * 0.9_f64.powf(wrong_count as f64) / n as f64;

            if score > best_score {
                best_score = score;
                best_n = n;
            }
            eprintln!("M = {}, eps = {}, n = {}, score = {}", m, eps, n, score);
        } else {
            eprintln!("[error] {} does not exist.", &file_path);
        }
    }

    println!("Result = {},{},{}", m, e, best_n);
    eprintln!("M = {}, eps = {}, best_n = {}", m, eps, best_n);
}
