use crate::{
    graph::{calc_graph_similarity, Graph},
    util::rnd,
};

pub fn create_initial_graphs(n: usize, m: usize) -> Vec<Graph> {
    let mut graphs = vec![];
    let max_graph_size = n * (n - 1) / 2;
    for i in 0..m {
        let mut graph_raw_format = vec![false; max_graph_size];
        let graph_size = max_graph_size * i / (m - 1);
        for _ in 0..graph_size {
            let mut i: usize = rnd::gen_range(0, max_graph_size);
            while graph_raw_format[i] == true {
                i = rnd::gen_range(0, max_graph_size);
            }
            graph_raw_format[i] = true;
        }
        graphs.push(Graph::from_vec_format(n, graph_raw_format));
    }
    return graphs;
}

pub fn create_optimal_graphs(n: usize, m: usize, iter_count: usize) -> State {
    // M個のグラフを初期化する
    let graphs = create_initial_graphs(n, m);
    let mut state = State::new(graphs);

    // TODO: 焼きなまし
    for _ in 0..iter_count {
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

    state
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
        state.score = state.calc_score();
        state
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
                self.update_similarity_matrix(*graph_index);

                self.score = self.calc_score();
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

    fn update_similarity_matrix(&mut self, updated_graph_index: usize) {
        // TODO: 全てのグラフと試すのではなく、何個かサンプリングする
        for j in 0..self.graphs.len() {
            if updated_graph_index == j {
                continue;
            }
            let similarity =
                calc_graph_similarity(&self.graphs[updated_graph_index], &self.graphs[j]);
            self.similarity_matrix[updated_graph_index][j] = similarity;
            self.similarity_matrix[j][updated_graph_index] = similarity;
        }
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

    fn calc_score(&self) -> i64 {
        // 各グラフについて、最も近いグラフとの距離の総和
        // 大きいほどよい
        let mut score = 0;
        for i in 0..self.graphs.len() {
            let mut min_dist = i64::MAX;
            for j in 0..self.graphs.len() {
                if i == j {
                    continue;
                }
                min_dist = i64::min(min_dist, self.similarity_matrix[i][j]);
            }
            score += min_dist;
        }
        score
    }

    pub fn output(&self) {
        for graph in &self.graphs {
            println!("{}", graph.to_raw_format());
        }
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
    let graphs = create_initial_graphs(n, m);
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
        assert_eq!(state.score, state.calc_score());
    }

    copied_state_greedy.update_similarity_matrix_slow();
    copied_state_greedy.score = copied_state_greedy.calc_score();

    assert_eq!(state, copied_state_greedy);

    for command in commands.into_iter().rev() {
        state.reverse_command(&command);
    }

    assert_eq!(state, copied_state);
}
