#[derive(Debug)]
pub struct Graph {
    n: usize,
    degrees: Vec<usize>,
    edges: Vec<bool>,
    pairs: Vec<(usize, usize)>,
}

impl Graph {
    pub fn from_vec_format(n: usize, vec_format: Vec<bool>) -> Graph {
        let mut degrees = vec![0; n];
        let mut edges = vec_format;
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

    pub fn has_edge(&self, edge_index: usize) -> bool {
        self.edges[edge_index]
    }

    // 現在辺があるかどうかを返す
    pub fn toggle_edge(&mut self, edge_index: usize) -> bool {
        debug_assert!(edge_index < self.edges.len());

        self.edges[edge_index] = !self.edges[edge_index];
        let (v, u) = self.pairs[edge_index];
        if self.edges[edge_index] {
            self.degrees[v] += 1;
            self.degrees[u] += 1;
        } else {
            self.degrees[v] -= 1;
            self.degrees[u] -= 1;
        }
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
        let d = a_degrees[i] - b_degrees[i];
        sum += d * d;
    }

    return sum as i64;
}
