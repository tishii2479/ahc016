use crate::util::rnd;

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

        // 初期化時にはシミュレーションは行わず、必要な時にだけ行う
        let simulated_degrees = vec![];

        Graph {
            n,
            edge_count,
            degrees,
            simulated_degrees,
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

// 次数の差の平方和をグラフの類似度とした時の、類似度を返す関数
pub fn calc_degrees_similarity(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    debug_assert_eq!(a.len(), b.len());

    let mut score = 0.;

    for i in 0..a.len() {
        let d = a[i] - b[i];
        score += d * d;
    }

    score
}

pub fn vertex_indicies_to_pair_index(n: usize, v1: usize, v2: usize) -> usize {
    let mn = usize::min(v1, v2);
    let mx = usize::max(v1, v2);

    ((n - 1) + (n - mn)) * mn / 2 + (mx - mn - 1)
}
