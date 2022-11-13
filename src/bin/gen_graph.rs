use ahc016::gen::create_optimal_graphs;
use std::ops::RangeInclusive;

// 各M, eps, Nについてグラフを構築し、出力する
fn main() {
    const M_RANGE: RangeInclusive<usize> = 10..=100;
    const N_RANGE: RangeInclusive<usize> = 4..=100;
    const EPS_RANGE: RangeInclusive<usize> = 0..=40;

    const ITER_COUNT: usize = 10000;

    for n in N_RANGE {
        for m in M_RANGE {
            for e in EPS_RANGE {
                let eps = e as f64 * 0.01;

                // グラフを作成して出力する
                let state = create_optimal_graphs(n, m, eps, ITER_COUNT);
                state.format_to_string();
            }
        }
    }
}
