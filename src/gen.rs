#[allow(unused_imports)]
use std::{fs::File, io::Write};

use crate::{
    graph::{calc_graph_similarity, calc_simulated_graph, vertex_indicies_to_pair_index, Graph},
    util::{rnd, time},
};

pub fn create_optimal_graphs_greedy(n: usize, m: usize, eps: f64, _time_limit: f64) -> Vec<Graph> {
    let mut graphs = vec![];
    let max_graph_size = n * (n - 1) / 2;

    const SIMULATE_TRIAL_COUNT: usize = 15;

    for i in 0..m {
        // TODO: graph_raw_formatを使い回す、ボトルネックではないので優先度は低い
        let border = n;
        let edge_width = max_graph_size - border;
        let graph_size = border / 2 + edge_width * i / (m - 1);

        let fs: Vec<fn(usize, usize, usize, usize) -> Vec<bool>> = if eps <= 0.3 || m <= 40 {
            vec![f1, f2, f4]
        } else {
            vec![f1, f2, f3, f4]
        };
        let f = fs[i % fs.len()];
        let mut graph = Graph::from_vec_format(n, f(graph_size, max_graph_size, n, m));
        calc_simulated_graph(&mut graph, m, eps, SIMULATE_TRIAL_COUNT);
        graphs.push(graph);
    }
    return graphs;
}

pub fn create_optimal_graphs(n: usize, m: usize, eps: f64, time_limit: f64) -> Vec<Graph> {
    let start_time = time::elapsed_seconds();
    const SIMULATE_TRIAL_COUNT: usize = 20;
    const CANDIDATE_COUNT: usize = 2;

    let fs: Vec<fn(usize, usize, usize, usize) -> Vec<bool>> = vec![f1, f2, f3, f4, f5];

    let mut selected = vec![0; m];
    for i in 0..m {
        selected[i] = i % 3;
    }

    let mut graphs = vec![];
    let mut groups = vec![vec![]; m];

    let max_graph_size = n * (n - 1) / 2;
    // TODO: borderの大きさの調整
    let border = n;
    let edge_width = max_graph_size - border;

    for i in 0..m {
        for f in &fs {
            let graph_size = border / 2 + edge_width * i / (m - 1);
            let mut graph = Graph::from_vec_format(n, f(graph_size, max_graph_size, n, m));
            calc_simulated_graph(&mut graph, m, eps, SIMULATE_TRIAL_COUNT);
            graphs.push(graph);
        }
        let group = ((i * fs.len())..((i + 1) * fs.len())).collect();
        groups[i] = group;
    }
    for i in 0..m {
        for f in &fs {
            let base_graph_size = border / 2 + edge_width * i / (m - 1);
            let d = if edge_width / m > 1 {
                // TODO: 調整する
                edge_width / m
            } else {
                2
            };
            for c in 0..CANDIDATE_COUNT {
                let diff = rnd::gen_range(1, d);
                let graph_size = if c % 2 == 0 {
                    base_graph_size - diff
                } else {
                    base_graph_size + diff
                };
                if graph_size > max_graph_size {
                    continue;
                }
                let mut graph = Graph::from_vec_format(n, f(graph_size, max_graph_size, n, m));
                if graph.edge_count != graph_size {
                    // fを使ってgraph_sizeのグラフが作れない場合があるので、その時はgraphsに追加しない
                    continue;
                }
                calc_simulated_graph(&mut graph, m, eps, SIMULATE_TRIAL_COUNT);
                groups[i].push(graphs.len());
                graphs.push(graph);
            }
        }
    }

    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());

    let mut state = State::new(graphs, selected.clone(), groups, m, eps);
    let mut iter_count = 0;
    let start_temp: f64 = state.score / 5.;
    let end_temp: f64 = state.score / 1000.;

    let start_score = state.score;
    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());

    const LOOP_INTERVAL: usize = 100;
    let mut progress;
    let mut temp = 0.;

    while iter_count % LOOP_INTERVAL != 0 || time::elapsed_seconds() < start_time + time_limit {
        let current_score = state.score;
        if iter_count % LOOP_INTERVAL == 0 {
            progress = (time::elapsed_seconds() - start_time) / time_limit;
            temp = start_temp.powf(1. - progress) * end_temp.powf(progress);
        }

        let mut command: Command;

        loop {
            let p = rnd::nextf();
            if p < 0.4 {
                let move_index = rnd::gen_range(0, m);
                let from_graph_index = state.selected[move_index];
                let to_graph_index = { rnd::gen_range(0, state.groups[move_index].len()) };
                command = Command::Change {
                    move_index,
                    from_graph_index,
                    to_graph_index,
                };
            } else if p < 0.6 {
                let move_index = rnd::gen_range(0, m - 1);
                command = Command::Swap {
                    move_index1: move_index,
                    move_index2: move_index + 1,
                };
            } else if p < 0.8 {
                command = Command::Swap {
                    move_index1: rnd::gen_range(0, m),
                    move_index2: rnd::gen_range(0, m),
                };
            } else {
                let move_index1 = rnd::gen_range(0, m);
                let from_graph_index1 = state.selected[move_index1];
                let to_graph_index1 = { rnd::gen_range(0, state.groups[move_index1].len()) };
                let move_index2 = rnd::gen_range(0, m);
                let from_graph_index2 = state.selected[move_index2];
                let to_graph_index2 = { rnd::gen_range(0, state.groups[move_index2].len()) };
                command = Command::ChangeAdj {
                    move_index1,
                    from_graph_index1,
                    to_graph_index1,
                    move_index2,
                    from_graph_index2,
                    to_graph_index2,
                };
            }

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
        } else {
            // 不採用、ロールバック
            state.reverse_command(&command);
        }

        iter_count += 1;
    }

    eprintln!("start_score:    {}", start_score);
    eprintln!("final_score:    {}", state.score);
    eprintln!("iter_count:     {}", iter_count);
    eprintln!("final selected: {:?}", state.selected);

    if start_score > state.score {
        eprintln!(
            "became worse: {} -> {}, {} {}",
            start_score, state.score, m, eps
        );
        state.selected = selected;
    }

    let mut graphs = vec![];
    for (i, e) in state.selected.iter().enumerate() {
        graphs.push(state.graphs[state.groups[i][*e]].clone());
    }
    graphs
}

#[derive(Debug, Clone)]
enum Command {
    Change {
        move_index: usize,
        from_graph_index: usize,
        to_graph_index: usize,
    },
    Swap {
        move_index1: usize,
        move_index2: usize,
    },
    ChangeAdj {
        move_index1: usize,
        from_graph_index1: usize,
        to_graph_index1: usize,
        move_index2: usize,
        from_graph_index2: usize,
        to_graph_index2: usize,
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
    fn new(
        graphs: Vec<Graph>,
        selected: Vec<usize>,
        groups: Vec<Vec<usize>>,
        m: usize,
        eps: f64,
    ) -> State {
        let similarity_matrix = vec![vec![0.; graphs.len()]; graphs.len()];
        let mut state = State {
            score: 0.,
            graphs,
            selected,
            groups,
            similarity_matrix,
        };
        state.update_similarity_matrix_slow(m, eps);
        state.score = state.calc_score();
        state
    }

    fn can_perform_command(&mut self, command: &Command) -> bool {
        match command {
            Command::Change {
                move_index,
                from_graph_index: _,
                to_graph_index,
            } => *to_graph_index < self.groups[*move_index].len(),
            Command::Swap {
                move_index1,
                move_index2,
            } => {
                self.selected[*move_index1] < self.groups[*move_index2].len()
                    && self.selected[*move_index2] < self.groups[*move_index1].len()
            }
            Command::ChangeAdj {
                move_index1,
                from_graph_index1: _,
                to_graph_index1,
                move_index2,
                from_graph_index2: _,
                to_graph_index2,
            } => {
                *to_graph_index1 < self.groups[*move_index1].len()
                    && *to_graph_index2 < self.groups[*move_index2].len()
            }
        }
    }

    fn perform_command(&mut self, command: &Command) {
        match command {
            Command::Change {
                move_index,
                from_graph_index: _,
                to_graph_index,
            } => {
                self.selected[*move_index] = *to_graph_index;
            }
            Command::Swap {
                move_index1,
                move_index2,
            } => {
                self.selected.swap(*move_index1, *move_index2);
            }
            Command::ChangeAdj {
                move_index1,
                from_graph_index1: _,
                to_graph_index1,
                move_index2,
                from_graph_index2: _,
                to_graph_index2,
            } => {
                self.selected[*move_index1] = *to_graph_index1;
                self.selected[*move_index2] = *to_graph_index2;
            }
        }
    }

    fn reverse_command(&mut self, command: &Command) {
        match command {
            Command::Change {
                move_index,
                from_graph_index,
                to_graph_index: _,
            } => {
                self.selected[*move_index] = *from_graph_index;
            }
            Command::Swap {
                move_index1,
                move_index2,
            } => {
                self.selected.swap(*move_index1, *move_index2);
            }
            Command::ChangeAdj {
                move_index1,
                from_graph_index1,
                to_graph_index1: _,
                move_index2,
                from_graph_index2,
                to_graph_index2: _,
            } => {
                self.selected[*move_index1] = *from_graph_index1;
                self.selected[*move_index2] = *from_graph_index2;
            }
        }
    }

    fn update_similarity_matrix_slow(&mut self, m: usize, eps: f64) {
        for i in 0..self.graphs.len() {
            for j in 0..self.graphs.len() {
                if i == j {
                    self.similarity_matrix[i][j] = 0.;
                } else {
                    self.similarity_matrix[i][j] =
                        calc_graph_similarity(&self.graphs[i], &self.graphs[j], m, eps);
                }
            }
        }
    }

    fn calc_score(&self) -> f64 {
        // TODO: 調整
        // CONSIDER_COUNTはMを超えてはならない
        const CONSIDER_COUNT: usize = 10;
        // 各グラフ間の距離の総和
        // 大きいほどよい
        let mut min_dists = vec![];
        for (i, ie) in self.selected.iter().enumerate() {
            let mut min_dist = 1e10;
            let i_idx = self.groups[i][*ie];
            for (j, je) in self.selected.iter().enumerate() {
                if i == j {
                    continue;
                }
                let j_idx = self.groups[j][*je];
                min_dist = f64::min(min_dist, self.similarity_matrix[i_idx][j_idx]);
            }
            min_dists.push(min_dist);
        }
        min_dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut score = 0.;
        for i in 0..CONSIDER_COUNT {
            score += min_dists[i];
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

// f1とf2の中間
#[allow(unused_variables, dead_code)]
fn f3(graph_size: usize, max_graph_size: usize, n: usize, m: usize) -> Vec<bool> {
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

// なるべく均等に辺を貼る、斜め
#[allow(unused_variables, dead_code)]
fn f4(graph_size: usize, max_graph_size: usize, n: usize, m: usize) -> Vec<bool> {
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

// 真ん中を開けるように作る
#[allow(unused_variables, dead_code)]
fn f5(graph_size: usize, max_graph_size: usize, n: usize, m: usize) -> Vec<bool> {
    let mut graph_raw_format = vec![false; max_graph_size];
    let mut counter = 0;

    for i in 0..n {
        let sz = usize::max(n / 5, 1);
        let v = sz * (i / sz + 1);
        let l = usize::min(usize::max(i + 1, v), n);
        for j in l..n {
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
