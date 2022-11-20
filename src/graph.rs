use crate::util::rnd;

const USE_SQUARE_LIMIT: f64 = 10.;

#[derive(Debug, Clone)]
pub struct Graph {
    pub n: usize,
    pub edge_count: usize,
    pub degrees: Vec<f64>,
    pub simulated_degrees: Vec<f64>,
    pub simulated_squares: Vec<f64>,
    // TODO: BitSetにかえる
    edges: Vec<bool>,
    // TODO: 取り除く
    pairs: Vec<(usize, usize)>,
}

impl Graph {
    pub fn from_vec_format(n: usize, vec_format: Vec<bool>) -> Graph {
        let mut degrees = vec![0.; n];
        let edges = vec_format;
        let mut pairs = vec![(0, 0); edges.len()];

        let mut it = 0;
        let mut edge_count = 0;

        for i in 0..n {
            for j in i + 1..n {
                if edges[it] {
                    degrees[i] += 1.;
                    degrees[j] += 1.;
                    edge_count += 1;
                }
                pairs[it] = (i, j);
                it += 1;
            }
        }

        // 初期化時にはシミュレーションは行わず、必要な時にだけ行う
        let simulated_degrees = vec![];
        let simulated_squares = vec![];

        Graph {
            n,
            edge_count,
            degrees,
            simulated_degrees,
            simulated_squares,
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
            self.degrees[v] += 1.;
            self.degrees[u] += 1.;
        } else {
            self.degrees[v] -= 1.;
            self.degrees[u] -= 1.;
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

pub fn calc_simulated_graph(graph: &mut Graph, m: usize, eps: f64, trial: usize) {
    let mut square_edge_counts = vec![];
    let mut simulated_degrees = vec![0.; graph.n];

    for _ in 0..trial {
        // ISSUE: cloneが重い
        let mut sim_graph = graph.clone();
        operate_toggle(&mut sim_graph, eps);

        let mut degrees = sim_graph.degrees.clone();
        degrees.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for i in 0..graph.n {
            simulated_degrees[i] += degrees[i];
        }

        // m*epsがUSE_SQUARE_LIMIT以下ならcalc_simulated_squareはしない
        if m as f64 * eps <= USE_SQUARE_LIMIT {
            continue;
        }
        let edge_counts = calc_simulated_square(&sim_graph);
        if square_edge_counts.len() == 0 {
            square_edge_counts = vec![0.; edge_counts.len()];
        }
        for i in 0..edge_counts.len() {
            square_edge_counts[i] += edge_counts[i];
        }
    }
    for i in 0..graph.n {
        simulated_degrees[i] /= trial as f64;
    }
    for i in 0..square_edge_counts.len() {
        square_edge_counts[i] /= trial as f64;
    }

    graph.simulated_degrees = simulated_degrees;
    graph.simulated_squares = square_edge_counts;
}

pub fn calc_simulated_square(graph: &Graph) -> Vec<f64> {
    let mut rank: Vec<usize> = (0..graph.degrees.len()).collect();
    rank.sort_by(|i, j| graph.degrees[*i].partial_cmp(&graph.degrees[*j]).unwrap());

    const DIV: usize = 5;
    let mut edge_counts = vec![0.; DIV * DIV];

    for i in 0..DIV {
        for j in 0..DIV {
            // eprintln!("{}, {}", i, j);
            let x = graph.n * i / DIV;
            let y = graph.n * j / DIV;
            let w = graph.n / DIV;
            let h = graph.n / DIV;
            let mut edge_count = 0.;
            for i in y..(y + h) {
                for j in x..(x + w) {
                    if i == j {
                        continue;
                    }
                    if i >= graph.n || j >= graph.n {
                        continue;
                    }
                    let p = vertex_indicies_to_pair_index(graph.n, rank[i], rank[j]);
                    edge_count += if graph.edges[p] { 1. } else { 0. };
                }
            }
            edge_counts[i * DIV + j] += edge_count;
        }
    }

    edge_counts
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

// a.simulated_squares、b.simulated_squaresは同じ長さである必要がある
fn calc_matrix_similarity(a: &Graph, b: &Graph) -> f64 {
    let mut score = 0.;

    for i in 0..a.simulated_squares.len() {
        let d = a.simulated_squares[i] - b.simulated_squares[i];
        score += d * d;
    }

    score
}

// 次数の差の平方和をグラフの類似度とした時の、類似度を返す関数
// a.simulated_degrees、b.simulated_degreesは同じ長さ、ソート済みである必要がある
fn calc_simulated_degrees_similarity(a: &Graph, b: &Graph) -> f64 {
    let mut score = 0.;

    for i in 0..a.simulated_degrees.len() {
        let d = a.simulated_degrees[i] - b.simulated_degrees[i];
        score += d * d;
    }

    score
}

// グラフの類似度を計算する関数
// 値が小さいほど類似している
pub fn calc_graph_similarity(a: &Graph, b: &Graph, m: usize, eps: f64) -> f64 {
    let degree_similarity = calc_simulated_degrees_similarity(&a, &b);
    // TODO: 係数の調整
    // m*epsがUSE_SQUARE_LIMIT以下ならcalc_matrix_similarityはしない
    if m as f64 * eps <= USE_SQUARE_LIMIT {
        degree_similarity
    } else {
        degree_similarity + calc_matrix_similarity(&a, &b) * (eps * 0.15)
    }
}

pub fn vertex_indicies_to_pair_index(n: usize, v1: usize, v2: usize) -> usize {
    let mn = usize::min(v1, v2);
    let mx = v1 + v2 - mn;

    ((n - 1) + (n - mn)) * mn / 2 + (mx - mn - 1)
}
