use ahc016::gen::create_optimal_graphs;
use std::{
    fs::{self, File},
    io::Write,
    ops::RangeInclusive,
    path::Path,
};

// 各M, eps, Nについてグラフを構築し、出力する
fn main() {
    // WARN: 正しくは 4..=100、一時的にNの数を小さくしている
    const N_RANGE: RangeInclusive<usize> = 10..=100;

    const M_RANGE: RangeInclusive<usize> = 50..=100;
    const EPS_RANGE: RangeInclusive<usize> = 10..=40;

    const ITER_COUNT: usize = 10000;

    for m in M_RANGE {
        for e in EPS_RANGE {
            let eps = e as f64 * 0.01;
            eprintln!("Creating M = {}, eps = {}...", m, eps);

            // WARN: 正しくは 4..=100、一時的にNの数を小さくしている
            let output_dir = format!("data/graphs/{m}_{e}/");
            if !Path::exists(Path::new(&output_dir)) {
                fs::create_dir_all(&output_dir).unwrap();
            }

            for n in N_RANGE.step_by(10) {
                // グラフを作成して出力する
                let state = create_optimal_graphs(n, m, eps, ITER_COUNT);
                let file_path = format!("data/graphs/{m}_{e}/{n}.txt");
                let mut file = File::create(file_path).unwrap();
                write!(file, "{}", state.format_to_string().trim()).unwrap();
            }
        }
    }
}
