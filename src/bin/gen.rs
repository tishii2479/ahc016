use ahc016::gen_graph::create_optimal_graphs;
use proconio::input;
use std::ops::RangeInclusive;

fn main() {
    const N_RANGE: RangeInclusive<usize> = 10..=10;
    const ITER_COUNT: usize = 10000;

    input! {
        m: usize,
        _: f64
    }

    for n in N_RANGE {
        // グラフを作成して出力する
        let state = create_optimal_graphs(n, m, ITER_COUNT);
        state.output();
    }
}
