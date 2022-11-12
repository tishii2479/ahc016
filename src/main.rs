use proconio::input;
mod graph;
mod util;

fn main() {
    const QUERY_COUNT: usize = 100;
    input! {
        M: usize,
        eps: String
    }

    // M, epsに対応するグラフを出力する

    // 各クエリを処理する
    for _ in 0..QUERY_COUNT {
        input! {
            h: String
        }

        // hとGとの類似度を求め、類似度が最大のGを出力する
    }
}
