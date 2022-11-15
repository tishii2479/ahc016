use ahc016::gen::create_optimal_graphs;
use std::{
    env,
    fs::{self, File},
    io::Write,
    ops::RangeInclusive,
    path::Path,
};

// 各M, eps, Nについてグラフを構築し、出力する
fn main() {
    // WARN: 正しくは 4..=100、一時的にNの数を小さくしている
    const N_RANGE: RangeInclusive<usize> = 10..=100;

    let args: Vec<String> = env::args().collect();
    let m = args[1].parse::<usize>().unwrap();
    let eps = args[2].parse::<f64>().unwrap();

    const TIME_LIMIT: f64 = 1.;
    let e = ((eps * 100.) as i64).to_string();

    eprintln!("Creating M = {}, eps = {}...", m, eps);

    // WARN: 正しくは 4..=100、一時的にNの数を小さくしている
    let output_dir = format!("data/graphs/{m}_{e}/");
    if !Path::exists(Path::new(&output_dir)) {
        fs::create_dir_all(&output_dir).unwrap();
    }

    for n in N_RANGE.step_by(10) {
        // グラフを作成して出力する
        let graphs = create_optimal_graphs(n, m, eps, TIME_LIMIT);
        let file_path = format!("data/graphs/{m}_{e}/{n}.txt");
        let mut file = File::create(file_path).unwrap();
        for graph in &graphs {
            write!(file, "{}", graph.to_raw_format().trim()).unwrap();
        }
    }
}
