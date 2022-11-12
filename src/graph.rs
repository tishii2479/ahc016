use crate::util::{generate_shuffled_permutation, rnd};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    n: usize,
    degrees: Vec<usize>,
    edges: Vec<bool>,
    pairs: Vec<(usize, usize)>,
}

impl Graph {
    pub fn from_vec_format(n: usize, vec_format: Vec<bool>) -> Graph {
        let mut degrees = vec![0; n];
        let edges = vec_format;
        let mut pairs = vec![(0, 0); edges.len()];

        let mut it = 0;

        for i in 0..n {
            for j in i + 1..n {
                if edges[it] {
                    degrees[i] += 1;
                    degrees[j] += 1;
                }
                pairs[it] = (i, j);
                it += 1;
            }
        }

        Graph {
            n,
            degrees,
            edges,
            pairs,
        }
    }

    pub fn from_raw_format(n: usize, raw_format: &str) -> Graph {
        let vec_format = raw_format
            .chars()
            .map(|c| if c == '1' { true } else { false })
            .collect();
        Graph::from_vec_format(n, vec_format)
    }

    pub fn to_raw_format(&self) -> String {
        self.edges
            .iter()
            .map(|&e| if e { "1" } else { "0" })
            .collect()
    }

    pub fn set_edge(&mut self, edge_index: usize, value: bool) {
        if self.edges[edge_index] == value {
            return;
        }
        self.edges[edge_index] = value;

        let (v, u) = self.pairs[edge_index];
        if self.edges[edge_index] {
            self.degrees[v] += 1;
            self.degrees[u] += 1;
        } else {
            self.degrees[v] -= 1;
            self.degrees[u] -= 1;
        }
    }

    pub fn has_edge(&self, edge_index: usize) -> bool {
        self.edges[edge_index]
    }

    // 現在辺があるかどうかを返す
    pub fn toggle_edge(&mut self, edge_index: usize) -> bool {
        debug_assert!(edge_index < self.edges.len());

        self.set_edge(edge_index, !self.edges[edge_index]);

        self.edges[edge_index]
    }
}

// 次数の差の平方和をグラフの類似度とした時の、類似度を返す関数
pub fn calc_graph_similarity(a: &Graph, b: &Graph) -> i64 {
    let mut a_degrees = a.degrees.clone();
    let mut b_degrees = b.degrees.clone();
    a_degrees.sort();
    b_degrees.sort();

    debug_assert_eq!(a.n, b.n);

    let mut sum = 0;

    for i in 0..a.n {
        let d = a_degrees[i] as i64 - b_degrees[i] as i64;
        sum += d * d;
    }

    return sum;
}

// グラフを同じ形にするために必要な操作回数を類似度とした時の、類似度を返す関数
// 山登りによって頂点の対応付けを行い、最適化された時の必要な操作回数
pub fn calc_graph_similarity_with_hill_climbing(a: &Graph, b: &Graph, iter_count: usize) -> i64 {
    let n = a.n;
    let mut p = generate_shuffled_permutation(n);
    let current_score = 0;

    // TODO: 焼きなまし
    for _ in 0..iter_count {
        let v = rnd::gen_range(0, n);
        let u = rnd::gen_range(0, n);

        let new_score = 0;

        if new_score > current_score {
            // 採用
        } else {
            // 不採用、ロールバック
        }
    }

    current_score
}

// グラフを同じ形にするために必要な操作回数を類似度とした時の、類似度を返す関数
// ビームサーチによって頂点の対応付けを行い、最適化された時の必要な操作回数
// pub fn calc_graph_similarity_with_beam_search(a: &Graph, b: &Graph) -> i64 {
//     todo!();
// }
