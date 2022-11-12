pub struct Graph {
    n: usize,
    degrees: Vec<usize>,
    edges: Vec<Vec<bool>>,
}

impl Graph {
    pub fn from_vec_format(n: usize, vec_format: &Vec<bool>) -> Graph {
        let mut degrees = vec![0; n];
        let mut edges = vec![vec![false; n]; n];

        let mut it = 0;

        for i in 0..n {
            for j in i + 1..n {
                if vec_format[it] {
                    edges[i][j] = true;
                    edges[j][i] = true;

                    degrees[i] += 1;
                    degrees[j] += 1;
                }
                it += 1;
            }
        }

        Graph {
            n: n,
            degrees: degrees,
            edges: edges,
        }
    }

    pub fn from_raw_format(n: usize, raw_format: &str) -> Graph {
        todo!();
    }

    pub fn to_raw_format(&self) -> String {
        todo!();
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
