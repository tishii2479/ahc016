use crate::{
    graph::{calc_graph_similarity, vertex_indicies_to_pair_index, Graph},
    util::{generate_shuffled_permutation, rnd, time},
};

// ex: 9 9 0 0 0 0
#[allow(unused_variables, dead_code)]
fn f1(graph_size: usize, max_graph_size: usize, n: usize, m: usize) -> Vec<bool> {
    let mut graph_raw_format = vec![false; max_graph_size];
    for j in 0..graph_size {
        graph_raw_format[j] = true;
    }
    graph_raw_format
}

// ex: 5 5 5 5 0 0
#[allow(unused_variables, dead_code)]
fn f2(graph_size: usize, max_graph_size: usize, n: usize, m: usize) -> Vec<bool> {
    let mut graph_raw_format = vec![false; max_graph_size];
    let mut counter = 0;
    for j in 1..n {
        for i in 0..j {
            if counter >= graph_size {
                break;
            }
            let p = vertex_indicies_to_pair_index(n, i, j);
            graph_raw_format[p] = true;
            counter += 1;
        }
    }
    graph_raw_format
}

// ex: 3 3 3 2 2 2
#[allow(unused_variables, dead_code)]
fn f3(graph_size: usize, max_graph_size: usize, n: usize, m: usize) -> Vec<bool> {
    let mut graph_raw_format = vec![false; max_graph_size];
    let mut counter = 0;
    for d in 1..n {
        for i in 0..n - d {
            if counter >= graph_size {
                break;
            }
            let j = i + d;
            let p = vertex_indicies_to_pair_index(n, i, j);
            graph_raw_format[p] = true;
            counter += 1;
        }
    }
    graph_raw_format
}

// ex: 5 5 2 2 2 0
#[allow(unused_variables, dead_code)]
fn f4(graph_size: usize, max_graph_size: usize, n: usize, m: usize) -> Vec<bool> {
    let mut graph_raw_format = vec![false; max_graph_size];
    let mut counter = 0;
    for j in 0..max_graph_size {
        if counter >= graph_size / 2 {
            break;
        }
        graph_raw_format[j] = true;
        counter += 1;
    }
    for j in 0..max_graph_size {
        if counter >= graph_size {
            break;
        }
        graph_raw_format[max_graph_size - j - 1] = true;
        counter += 1;
    }
    graph_raw_format
}

// ex: 5 5 2 2 2 0
#[allow(unused_variables, dead_code)]
fn f5(graph_size: usize, max_graph_size: usize, n: usize, m: usize) -> Vec<bool> {
    let mut graph_raw_format = vec![false; max_graph_size];
    let p = generate_shuffled_permutation(max_graph_size);
    for j in 0..graph_size {
        graph_raw_format[p[j]] = true;
    }
    graph_raw_format
}

pub fn create_initial_graphs(n: usize, m: usize, eps: f64) -> Vec<Graph> {
    let mut graphs = vec![];
    let max_graph_size = n * (n - 1) / 2;

    for i in 0..m {
        // TODO: graph_raw_formatを使い回す
        // TODO: 等間隔以外を試す
        // TODO: 下駄 の大きさを変える
        let is_extreme = eps >= 0.35 && m >= 70;
        let border = if is_extreme { 0 } else { n };
        let graph_size = border / 2 + (max_graph_size - border) * i / (m - 1);

        let fs: Vec<fn(usize, usize, usize, usize) -> Vec<bool>> = if eps <= 0.3 || m <= 40 {
            vec![f1, f2, f4]
        } else if is_extreme {
            vec![f1, f2, f3, f4]
        } else {
            vec![f1, f2, f4, f3, f1, f2, f4]
        };
        let f = fs[i % fs.len()];
        graphs.push(Graph::from_vec_format(
            n,
            f(graph_size, max_graph_size, n, m),
        ));
    }
    return graphs;
}

pub fn create_optimal_graphs(n: usize, m: usize, eps: f64, time_limit: f64) -> Vec<Graph> {
    let start_time = time::elapsed_seconds();

    // TODO: epsを考慮する
    // M個のグラフを初期化する
    let graphs = create_initial_graphs(n, m, eps);
    let mut state = State::new(graphs);

    // TODO: 焼きなまし
    // TODO: 時間管理を効率的に
    while time::elapsed_seconds() < start_time + time_limit {
        let current_score = state.score;

        // 辺を付け替える
        let mut command: Command;

        loop {
            command = Command::Swap {
                graph_index: rnd::gen_range(0, m),
                edge_index1: rnd::gen_range(0, n * (n - 1) / 2),
                edge_index2: rnd::gen_range(0, n * (n - 1) / 2),
            };

            if state.can_perform_command(&command) {
                state.perform_command(&command);
                break;
            }
        }

        // グラフの距離を計算する
        let new_score = state.score;

        if new_score > current_score {
            // 採用
        } else {
            // 不採用、ロールバック
            state.reverse_command(&command);
        }
    }

    state.graphs
}

#[derive(Debug, Clone)]
enum Command {
    // 2つの辺の接続を切り替える近傍
    // 辺の総数を不変にするため、2つの辺の接続の有無が異なる必要がある
    Swap {
        graph_index: usize,
        edge_index1: usize,
        edge_index2: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub score: i64,
    pub graphs: Vec<Graph>,
    pub similarity_matrix: Vec<Vec<i64>>,
}

impl State {
    fn new(graphs: Vec<Graph>) -> State {
        let similarity_matrix = vec![vec![0; graphs.len()]; graphs.len()];
        let mut state = State {
            score: 0,
            graphs,
            similarity_matrix,
        };
        state.update_similarity_matrix_slow();
        state.score = state.calc_score_slow();
        state
    }

    pub fn from_raw_format(n: usize, raw_format: &str) -> State {
        let graphs = raw_format
            .trim()
            .split(" ")
            .into_iter()
            .map(|x| Graph::from_raw_format(n, x))
            .collect();

        State::new(graphs)
    }

    fn can_perform_command(&mut self, command: &Command) -> bool {
        match command {
            Command::Swap {
                graph_index,
                edge_index1,
                edge_index2,
            } => {
                self.graphs[*graph_index].has_edge(*edge_index1)
                    != self.graphs[*graph_index].has_edge(*edge_index2)
            }
        }
    }

    fn perform_command(&mut self, command: &Command) {
        match command {
            Command::Swap {
                graph_index,
                edge_index1,
                edge_index2,
            } => {
                self.graphs[*graph_index].toggle_edge(*edge_index1);
                self.graphs[*graph_index].toggle_edge(*edge_index2);
                let score_diff = self.update_similarity_matrix(*graph_index);

                self.score += score_diff;
            }
        }
    }

    fn reverse_command(&mut self, command: &Command) {
        match command {
            Command::Swap {
                graph_index: _,
                edge_index1: _,
                edge_index2: _,
            } => {
                self.perform_command(command);
            }
        }
    }

    fn update_similarity_matrix(&mut self, updated_graph_index: usize) -> i64 {
        // TODO: 全てのグラフと試すのではなく、何個かサンプリングする
        let mut score_diff = 0;
        for j in 0..self.graphs.len() {
            if updated_graph_index == j {
                continue;
            }
            let similarity =
                calc_graph_similarity(&self.graphs[updated_graph_index], &self.graphs[j]);

            score_diff += 2 * (similarity - self.similarity_matrix[updated_graph_index][j]);

            self.similarity_matrix[updated_graph_index][j] = similarity;
            self.similarity_matrix[j][updated_graph_index] = similarity;
        }
        score_diff
    }

    fn update_similarity_matrix_slow(&mut self) {
        for i in 0..self.graphs.len() {
            for j in 0..self.graphs.len() {
                if i == j {
                    self.similarity_matrix[i][j] = 0;
                } else {
                    self.similarity_matrix[i][j] =
                        calc_graph_similarity(&self.graphs[i], &self.graphs[j]);
                }
            }
        }
    }

    fn calc_score_slow(&self) -> i64 {
        // 各グラフ間の距離の総和
        // 大きいほどよい
        let mut score = 0;
        for i in 0..self.graphs.len() {
            for j in 0..self.graphs.len() {
                if i == j {
                    continue;
                }
                score += self.similarity_matrix[i][j];
            }
        }
        score
    }

    pub fn format_to_string(&self) -> String {
        self.graphs
            .iter()
            .map(|g| g.to_raw_format() + " ")
            .collect()
    }

    pub fn dump_similarity(&self) {
        for i in 0..self.graphs.len() {
            for j in 0..self.graphs.len() {
                let similarity = calc_graph_similarity(&self.graphs[i], &self.graphs[j]);
                eprint!("{:4} ", similarity);
            }
            eprintln!();
        }
        eprintln!();
    }
}

#[test]
fn test_perform_reverse_swap_command() {
    let n = 5;
    let m = 5;
    let eps = 0.1;
    let graphs = create_initial_graphs(n, m, eps);
    let mut state = State::new(graphs);

    let mut commands = vec![];

    let copied_state = state.clone();
    let mut copied_state_greedy = state.clone();

    for _ in 0..20 {
        let graph_index = rnd::gen_range(0, m);
        let edge_index1 = rnd::gen_range(0, n * (n - 1) / 2);
        let edge_index2 = rnd::gen_range(0, n * (n - 1) / 2);
        let command = Command::Swap {
            graph_index,
            edge_index1,
            edge_index2,
        };

        if state.can_perform_command(&command) {
            state.perform_command(&command);
            commands.push(command);

            copied_state_greedy.graphs[graph_index].toggle_edge(edge_index1);
            copied_state_greedy.graphs[graph_index].toggle_edge(edge_index2);
        }
        assert_eq!(state.score, state.calc_score_slow());
    }

    copied_state_greedy.update_similarity_matrix_slow();
    copied_state_greedy.score = copied_state_greedy.calc_score_slow();

    assert_eq!(state, copied_state_greedy);

    for command in commands.into_iter().rev() {
        state.reverse_command(&command);
    }

    assert_eq!(state, copied_state);
}
