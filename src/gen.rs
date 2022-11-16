use crate::{
    graph::{
        calc_degrees_similarity, calc_simulated_degrees, vertex_indicies_to_pair_index, Graph,
    },
    util::{rnd, time},
};

pub fn create_optimal_graphs(n: usize, m: usize, eps: f64, _time_limit: f64) -> Vec<Graph> {
    let mut graphs = vec![];
    let max_graph_size = n * (n - 1) / 2;

    const SIMULATE_TRIAL_COUNT: usize = 100;

    for i in 0..m {
        // TODO: graph_raw_formatを使い回す
        // TODO: 等間隔以外を試す
        // TODO: 下駄 の大きさを変える
        let is_extreme = eps >= 0.35 && m >= 70;
        let border = if is_extreme { 0 } else { n };
        let graph_size = border / 2 + (max_graph_size - border) * i / (m - 1);

        let fs: Vec<fn(usize, usize, usize, usize) -> Vec<bool>> = if eps <= 0.3 || m <= 40 {
            vec![f1, f2, f4]
        } else {
            vec![f1, f2, f3, f4]
        };
        let f = fs[i % fs.len()];
        let mut graph = Graph::from_vec_format(n, f(graph_size, max_graph_size, n, m));
        graph.simulated_degrees = calc_simulated_degrees(&graph, eps, SIMULATE_TRIAL_COUNT);
        graphs.push(graph);
    }
    return graphs;
}

pub fn create_optimal_graphs2(n: usize, m: usize, eps: f64, time_limit: f64) -> Vec<Graph> {
    let start_time = time::elapsed_seconds();

    // TODO: epsを考慮する
    let mut graphs = vec![];
    let max_graph_size = n * (n - 1) / 2;
    let is_extreme = eps >= 0.35 && m >= 70;
    let border = if is_extreme { 0 } else { n };
    let fs: Vec<fn(usize, usize, usize, usize) -> Vec<bool>> = if eps <= 0.3 || m <= 40 {
        vec![f1, f2, f4]
    } else {
        vec![f1, f2, f3, f4]
    };

    let mut groups = vec![];
    for i in 0..m {
        for f in &fs {
            let graph_size = border / 2 + (max_graph_size - border) * i / (m - 1);
            let mut graph = Graph::from_vec_format(n, f(graph_size, max_graph_size, n, m));
            graph.simulated_degrees = calc_simulated_degrees(&graph, eps, SIMULATE_TRIAL_COUNT);
            graphs.push(graph);
            let group = ((i * fs.len())..((i + 1) * fs.len())).collect();
            groups.push(group);
        }
    }

    const SIMULATE_TRIAL_COUNT: usize = 100;
    let mut selected = vec![];
    for i in 0..m {
        selected.push(i % fs.len());
    }
    eprintln!("{:?}", selected);

    let mut state = State::new(graphs, selected, groups);
    let mut iter_count = 0;
    let start_temp: f64 = 100000.;
    let end_temp: f64 = 1000.;
    // let time_limit = 0.;

    // TODO: 焼きなまし
    // TODO: 時間管理を効率的に
    while time::elapsed_seconds() < start_time + time_limit {
        let current_score = state.score;
        let progress = (time::elapsed_seconds() - start_time) / time_limit;
        let temp = start_temp.powf(1. - progress) * end_temp.powf(progress);

        // 辺を付け替える
        let mut command: Command;

        loop {
            let move_index = rnd::gen_range(0, m);
            let from_graph_index = state.selected[move_index];
            let to_graph_index = { rnd::gen_range(0, state.groups[move_index].len()) };
            command = Command::Swap {
                move_index,
                from_graph_index,
                to_graph_index,
            };

            if state.can_perform_command(&command) {
                state.perform_command(&command);
                break;
            }
        }

        // グラフの距離を計算する
        let new_score = state.calc_score();
        let adopt = ((new_score - current_score) as f64 / temp).exp() > rnd::nextf();

        if adopt {
            // 採用
            state.score = new_score;
            eprintln!("{}", state.score);
        } else {
            // 不採用、ロールバック
            state.reverse_command(&command);
        }

        iter_count += 1;
    }

    eprintln!("final_score: {}", state.score);
    eprintln!("iter_count:  {}", iter_count);

    eprintln!("{:?}", state.selected);

    let mut graphs = vec![];
    for (i, e) in state.selected.iter().enumerate() {
        graphs.push(state.graphs[i * fs.len() + e].clone());
    }
    graphs
}

#[derive(Debug, Clone)]
enum Command {
    Swap {
        move_index: usize,
        from_graph_index: usize,
        to_graph_index: usize,
    },
}

#[derive(Debug, Clone)]
pub struct State {
    pub score: f64,
    pub graphs: Vec<Graph>,
    pub selected: Vec<usize>,
    pub groups: Vec<Vec<usize>>,
    pub similarity_matrix: Vec<Vec<f64>>,
}

impl State {
    fn new(graphs: Vec<Graph>, selected: Vec<usize>, groups: Vec<Vec<usize>>) -> State {
        let similarity_matrix = vec![vec![0.; graphs.len()]; graphs.len()];
        let mut state = State {
            score: 0.,
            graphs,
            selected,
            groups,
            similarity_matrix,
        };
        state.update_similarity_matrix_slow();
        state.score = state.calc_score();
        state
    }

    fn can_perform_command(&mut self, command: &Command) -> bool {
        match command {
            Command::Swap {
                move_index: _,
                from_graph_index,
                to_graph_index,
            } => from_graph_index != to_graph_index,
        }
    }

    fn perform_command(&mut self, command: &Command) {
        match command {
            Command::Swap {
                move_index,
                from_graph_index: _,
                to_graph_index,
            } => {
                self.selected[*move_index] = *to_graph_index;
            }
        }
    }

    fn reverse_command(&mut self, command: &Command) {
        match command {
            Command::Swap {
                move_index,
                from_graph_index,
                to_graph_index: _,
            } => {
                self.selected[*move_index] = *from_graph_index;
            }
        }
    }

    fn update_similarity_matrix_slow(&mut self) {
        for i in 0..self.graphs.len() {
            for j in 0..self.graphs.len() {
                if i == j {
                    self.similarity_matrix[i][j] = 0.;
                } else {
                    self.similarity_matrix[i][j] = calc_degrees_similarity(
                        &self.graphs[i].simulated_degrees,
                        &self.graphs[j].simulated_degrees,
                    );
                }
            }
        }
    }

    fn calc_score(&self) -> f64 {
        // 各グラフ間の距離の総和
        // 大きいほどよい
        let mut score = 0.;
        for (i, ie) in self.selected.iter().enumerate() {
            let mut min_dist = 1e10;
            for (j, je) in self.selected.iter().enumerate() {
                if i == j {
                    continue;
                }
                let i_idx = self.groups[i][*ie];
                let j_idx = self.groups[j][*je];
                min_dist = f64::min(min_dist, self.similarity_matrix[i_idx][j_idx]);
            }
            score += min_dist;
        }
        score
    }

    pub fn format_to_string(&self) -> String {
        self.graphs
            .iter()
            .map(|g| g.to_raw_format() + " ")
            .collect()
    }
}

// ウニグラフ
#[allow(unused_variables, dead_code)]
fn f1(graph_size: usize, max_graph_size: usize, n: usize, m: usize) -> Vec<bool> {
    let mut graph_raw_format = vec![false; max_graph_size];
    for j in 0..graph_size {
        graph_raw_format[j] = true;
    }
    graph_raw_format
}

// 完全グラフを徐々に大きくする
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

// なるべく均等に辺を貼る、斜め
// TODO: ちゃんと均等にする
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

// f1とf2の中間
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

// なるべく均等に辺を張る
#[allow(unused_variables, dead_code)]
fn f5(graph_size: usize, max_graph_size: usize, n: usize, m: usize) -> Vec<bool> {
    let mut graph_raw_format = vec![false; max_graph_size];
    let mut degrees = vec![0; n];
    let mut max_degree = 0;

    // TODO: 高速化
    for _ in 0..graph_size {
        let mut best_score = usize::MAX;
        let mut best_vs = (0, 0);
        for i in 0..n {
            for j in i + 1..n {
                let p = vertex_indicies_to_pair_index(n, i, j);
                if graph_raw_format[p] {
                    continue;
                }
                let score = degrees[i] + degrees[j];
                if score < best_score {
                    best_score = score;
                    best_vs = (i, j);
                }
            }
        }
        let (i, j) = best_vs;
        let p = vertex_indicies_to_pair_index(n, i, j);
        graph_raw_format[p] = true;
        degrees[i] += 1;
        degrees[j] += 1;
        max_degree = usize::max(max_degree, usize::max(degrees[i], degrees[j]));
    }
    graph_raw_format
}

// #[test]
// fn test_perform_reverse_swap_command() {
//     let n = 5;
//     let m = 10;
//     let eps = 0.1;
//     let graphs = create_optimal_graphs(n, m, eps, 1.);
//     let graph_count = graphs.len();
//     let selected = vec![0, 1, 2, 3, 4];
//     let mut state = State::new(graphs, selected);

//     let mut commands = vec![];

//     let copied_state = state.clone();

//     for _ in 0..20 {
//         let move_index = rnd::gen_range(0, m);
//         let from_graph_index = state.selected[move_index];
//         let to_graph_index = {
//             if (from_graph_index == graph_count - 1) || (from_graph_index > 0 && rnd::nextf() < 0.5)
//             {
//                 from_graph_index - 1
//             } else {
//                 from_graph_index + 1
//             }
//         };
//         let command = Command::Swap {
//             move_index,
//             from_graph_index,
//             to_graph_index,
//         };

//         if state.can_perform_command(&command) {
//             state.perform_command(&command);
//             commands.push(command);
//         }
//         assert_eq!(state.score, state.calc_score());
//     }

//     for command in commands.into_iter().rev() {
//         state.reverse_command(&command);
//     }

//     assert_eq!(state.score, copied_state.score);
// }
