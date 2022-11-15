use std::{fs::File, io::Write};

use crate::util::{generate_shuffled_permutation, rnd, Dsu};

#[derive(Debug, Clone)]
pub struct Graph {
    pub n: usize,
    pub edge_count: usize,
    pub degrees: Vec<usize>,
    pub simulated_degrees: Vec<f64>,
    // TODO: BitSetにかえる
    edges: Vec<bool>,
    // TODO: 取り除く
    pairs: Vec<(usize, usize)>,
}

impl Graph {
    pub fn from_vec_format(n: usize, vec_format: Vec<bool>) -> Graph {
        let mut degrees = vec![0; n];
        let edges = vec_format;
        let mut pairs = vec![(0, 0); edges.len()];

        let mut it = 0;
        let mut edge_count = 0;

        for i in 0..n {
            for j in i + 1..n {
                if edges[it] {
                    degrees[i] += 1;
                    degrees[j] += 1;
                    edge_count += 1;
                }
                pairs[it] = (i, j);
                it += 1;
            }
        }
        let sim_degrees = degrees.iter().map(|x| *x as f64).collect();

        Graph {
            n,
            edge_count,
            degrees,
            simulated_degrees: sim_degrees,
            edges,
            pairs,
        }
    }

    pub fn from_raw_format(n: usize, raw_format: &str) -> Graph {
        let vec_format = raw_format
            .trim()
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
        self.set_edge(edge_index, !self.edges[edge_index]);

        self.edges[edge_index]
    }

    pub fn calc_edge_count(&self) -> usize {
        let mut ret = 0;
        for e in &self.edges {
            if *e {
                ret += 1;
            }
        }
        ret
    }

    pub fn get_edge_size(&self) -> usize {
        self.edges.len()
    }
}

pub fn calc_simulated_degrees(graph: &Graph, eps: f64, trial: usize) -> Vec<f64> {
    let mut simulated_degrees = vec![0.; graph.n];
    for _ in 0..trial {
        let mut sim_graph = graph.clone();
        operate_toggle(&mut sim_graph, eps);
        let mut degrees = sim_graph.degrees.clone();
        degrees.sort();
        for i in 0..graph.n {
            simulated_degrees[i] += degrees[i] as f64;
        }
    }
    for i in 0..graph.n {
        simulated_degrees[i] /= trial as f64;
    }
    simulated_degrees
}

// epsの確率でgraphを変化させる
pub fn operate_toggle(graph: &mut Graph, eps: f64) {
    // TODO: maskに変えて、xorをとる
    for j in 0..graph.get_edge_size() {
        if rnd::nextf() < eps {
            graph.toggle_edge(j);
        }
    }
}

pub fn calc_connected_components_size(graph: &Graph) -> Vec<usize> {
    let mut uf = Dsu::new(graph.n);
    for i in 0..graph.edges.len() {
        if graph.edges[i] {
            let (v, u) = graph.pairs[i];
            uf.merge(v, u);
        }
    }
    let mut sizes = vec![];
    for i in 0..graph.n {
        if uf.leader(i) == i {
            sizes.push(uf.size(i));
        }
    }
    sizes.sort_by(|a, b| b.cmp(a));
    sizes
}

// 次数の差の平方和をグラフの類似度とした時の、類似度を返す関数
#[allow(unused)]
fn calc_degrees_hist_similarity(a: &Graph, b: &Graph) -> i64 {
    let mut a_hist = vec![0; a.n];
    let mut b_hist = vec![0; a.n];

    for e in &a.degrees {
        a_hist[*e] += 1;
    }
    for e in &b.degrees {
        b_hist[*e] += 1;
    }

    debug_assert_eq!(a.n, b.n);

    let mut score = 0;

    for i in 0..a.n {
        score += i64::min(a_hist[i], b_hist[i]);
    }

    score
}

// 次数の差の平方和をグラフの類似度とした時の、類似度を返す関数
pub fn calc_degrees_similarity(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    // TODO: degreesの管理を止める
    debug_assert_eq!(a.len(), b.len());

    let mut score = 0.;

    for i in 0..a.len() {
        let d = a[i] - b[i];
        score += d * d;
    }

    score
}

#[allow(unused)]
fn calc_connected_size_similarity(a: &Graph, b: &Graph) -> i64 {
    let a = calc_connected_components_size(&a);
    let b = calc_connected_components_size(&b);

    let mut score = 0;

    for i in 0..usize::max(a.len(), b.len()) {
        let ea = if i < a.len() { a[i] } else { 0 };
        let eb = if i < b.len() { b[i] } else { 0 };
        let d = ea as i64 - eb as i64;
        score += d * d;
    }

    score
}

// グラフを同じ形にするために必要な操作回数を類似度とした時の、類似度を返す関数
// 山登りによって頂点の対応付けを行い、最適化された時の必要な操作回数
pub fn calc_graph_similarity_with_sa(
    a: &Graph,
    b: &Graph,
    iter_count: usize,
    start_temp: f64,
    end_temp: f64,
    write_score_log: bool,
) -> i64 {
    let n = a.n;
    let mut p = generate_shuffled_permutation(n);

    // 操作回数を最小化する
    // 操作回数 := 切り替えが必要な辺の数
    let mut current_score = 1e10 as i64;
    let mut scores = vec![];

    for iter in 0..iter_count {
        let progress = iter as f64 / iter_count as f64;
        let temp = start_temp.powf(1. - progress) * end_temp.powf(progress);

        let e1 = rnd::gen_range(0, n);
        let e2 = rnd::gen_range(0, n);

        p.swap(e1, e2);

        let mut new_score = 0;

        // TODO: 差分更新
        for i in 0..a.edges.len() {
            let (v1, v2) = a.pairs[i];
            let (p_v1, p_v2) = (p[v1], p[v2]);
            let j = vertex_indicies_to_pair_index(n, p_v1, p_v2);
            if a.has_edge(i) != b.has_edge(j) {
                new_score += 1;
            }
        }

        let adopt = (-(new_score - current_score) as f64 / temp).exp() > rnd::nextf();
        if adopt {
            // 採用
            current_score = new_score;
        } else {
            // 不採用、ロールバック
            p.swap(e1, e2);
        }
        scores.push(current_score);
    }

    if write_score_log {
        let mut file = File::create("score.log").unwrap();
        writeln!(file, "{:?}", scores).unwrap();
    }

    current_score
}

// グラフを同じ形にするために必要な操作回数を類似度とした時の、類似度を返す関数
// ビームサーチによって頂点の対応付けを行い、最適化された時の必要な操作回数
// pub fn calc_graph_similarity_with_beam_search(a: &Graph, b: &Graph) -> i64 {
//     todo!();
// }

pub fn vertex_indicies_to_pair_index(n: usize, v1: usize, v2: usize) -> usize {
    let mn = usize::min(v1, v2);
    let mx = usize::max(v1, v2);

    ((n - 1) + (n - mn)) * mn / 2 + (mx - mn - 1)
}
