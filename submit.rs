pub mod gen {
    #[allow(unused_imports)]
    use std::{fs::File, io::Write};

    use crate::{
        graph::{
            calc_graph_similarity, calc_simulated_graph, vertex_indicies_to_pair_index, Graph,
        },
        util::{rnd, time},
    };

    pub fn create_optimal_graphs_greedy(
        n: usize,
        m: usize,
        eps: f64,
        _time_limit: f64,
    ) -> Vec<Graph> {
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
}
pub mod graph {
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
}
pub mod param {
    pub const BEST_N: [[usize; 41]; 91] = [
        [
            5, 5, 7, 7, 7, 8, 10, 10, 11, 11, 11, 12, 13, 14, 14, 15, 15, 15, 16, 18, 20, 20, 21,
            21, 22, 25, 27, 27, 28, 32, 34, 36, 37, 46, 48, 56, 61, 65, 68, 80, 86,
        ],
        [
            5, 6, 7, 7, 8, 9, 10, 11, 11, 11, 12, 13, 13, 15, 16, 16, 16, 16, 17, 19, 20, 22, 24,
            24, 24, 26, 28, 29, 30, 34, 36, 40, 42, 47, 49, 57, 61, 73, 79, 86, 90,
        ],
        [
            5, 6, 7, 8, 8, 9, 10, 11, 12, 12, 13, 13, 14, 15, 16, 16, 17, 18, 19, 20, 21, 24, 26,
            26, 26, 28, 29, 31, 32, 35, 37, 42, 45, 47, 51, 58, 61, 77, 82, 90, 93,
        ],
        [
            5, 6, 8, 9, 9, 10, 11, 12, 13, 13, 13, 13, 14, 15, 16, 17, 17, 18, 19, 20, 21, 24, 26,
            26, 26, 32, 35, 35, 36, 38, 40, 47, 51, 51, 53, 66, 74, 80, 84, 90, 95,
        ],
        [
            5, 7, 8, 9, 9, 10, 11, 13, 14, 14, 14, 14, 15, 15, 17, 17, 18, 19, 20, 21, 22, 25, 27,
            27, 27, 33, 35, 39, 39, 41, 42, 49, 53, 53, 53, 71, 77, 82, 85, 91, 95,
        ],
        [
            5, 7, 8, 9, 9, 10, 12, 13, 14, 14, 14, 14, 15, 17, 18, 18, 19, 22, 22, 22, 24, 26, 28,
            28, 29, 33, 36, 39, 41, 42, 43, 50, 53, 58, 61, 72, 78, 82, 87, 92, 95,
        ],
        [
            5, 7, 8, 9, 9, 10, 13, 13, 14, 15, 15, 15, 15, 17, 19, 19, 20, 23, 23, 24, 24, 27, 29,
            29, 29, 35, 37, 40, 43, 43, 44, 50, 53, 61, 65, 73, 78, 85, 88, 93, 96,
        ],
        [
            5, 7, 8, 9, 9, 10, 13, 14, 15, 15, 15, 15, 16, 18, 19, 21, 22, 24, 25, 26, 27, 28, 29,
            30, 31, 35, 38, 41, 44, 45, 46, 50, 53, 65, 71, 75, 79, 86, 90, 95, 97,
        ],
        [
            5, 8, 8, 9, 9, 11, 13, 14, 15, 16, 16, 16, 17, 19, 20, 21, 23, 24, 26, 26, 28, 28, 29,
            31, 32, 36, 38, 42, 45, 47, 48, 51, 55, 66, 72, 77, 80, 89, 90, 96, 98,
        ],
        [
            5, 8, 8, 9, 10, 11, 13, 14, 15, 17, 18, 18, 18, 19, 20, 22, 23, 25, 26, 27, 28, 28, 29,
            32, 34, 36, 38, 46, 48, 50, 51, 53, 55, 67, 72, 78, 81, 90, 93, 97, 98,
        ],
        [
            5, 8, 9, 10, 10, 11, 13, 15, 16, 17, 19, 19, 19, 19, 20, 23, 25, 26, 26, 28, 29, 30,
            30, 33, 36, 37, 39, 47, 48, 52, 53, 55, 57, 67, 73, 78, 81, 91, 95, 97, 98,
        ],
        [
            6, 8, 9, 10, 10, 11, 13, 15, 16, 17, 19, 19, 19, 21, 22, 24, 26, 26, 27, 29, 30, 30,
            31, 35, 38, 41, 43, 47, 50, 52, 53, 57, 60, 68, 74, 78, 81, 91, 95, 97, 98,
        ],
        [
            6, 8, 9, 10, 11, 11, 13, 15, 17, 18, 19, 19, 19, 21, 22, 24, 26, 27, 28, 30, 31, 32,
            33, 37, 38, 42, 45, 48, 50, 53, 54, 59, 62, 70, 74, 79, 83, 92, 96, 97, 98,
        ],
        [
            6, 8, 10, 10, 11, 12, 14, 15, 17, 18, 19, 19, 20, 21, 22, 25, 26, 28, 28, 33, 34, 35,
            35, 37, 39, 43, 45, 48, 51, 54, 57, 61, 64, 73, 78, 83, 86, 92, 96, 97, 98,
        ],
        [
            6, 8, 10, 11, 11, 12, 14, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 28, 29, 33, 35, 36,
            36, 38, 39, 43, 45, 48, 52, 55, 59, 63, 66, 76, 81, 86, 89, 93, 96, 97, 98,
        ],
        [
            6, 8, 10, 11, 12, 12, 14, 16, 17, 18, 19, 20, 21, 22, 23, 25, 27, 28, 30, 33, 35, 36,
            36, 38, 39, 43, 46, 50, 53, 58, 62, 68, 71, 79, 82, 90, 94, 94, 96, 98, 98,
        ],
        [
            6, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 21, 22, 22, 23, 25, 27, 29, 31, 33, 35, 36,
            37, 38, 40, 43, 46, 52, 54, 60, 63, 70, 72, 80, 83, 91, 95, 96, 96, 98, 98,
        ],
        [
            6, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 21, 22, 23, 24, 26, 27, 29, 31, 34, 35, 36,
            37, 39, 40, 44, 47, 52, 56, 60, 63, 72, 73, 81, 83, 91, 95, 96, 97, 98, 98,
        ],
        [
            6, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 21, 22, 23, 24, 26, 27, 29, 31, 34, 36, 37,
            38, 39, 40, 45, 48, 53, 57, 61, 64, 72, 74, 81, 83, 93, 95, 96, 97, 98, 99,
        ],
        [
            6, 9, 11, 12, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 26, 27, 29, 31, 34, 36, 37,
            38, 39, 41, 45, 48, 53, 57, 64, 66, 72, 75, 82, 84, 93, 96, 97, 97, 98, 99,
        ],
        [
            6, 9, 11, 12, 13, 14, 15, 16, 18, 19, 20, 22, 22, 24, 25, 26, 28, 29, 31, 35, 36, 37,
            39, 40, 41, 46, 49, 54, 57, 65, 68, 73, 76, 82, 85, 94, 96, 97, 98, 98, 99,
        ],
        [
            6, 9, 11, 12, 13, 15, 15, 17, 18, 20, 21, 22, 22, 24, 25, 27, 28, 30, 31, 35, 37, 37,
            39, 40, 41, 47, 50, 55, 58, 65, 69, 73, 78, 82, 85, 94, 96, 97, 98, 98, 99,
        ],
        [
            6, 9, 11, 12, 13, 15, 15, 17, 18, 20, 21, 22, 23, 24, 26, 28, 29, 31, 32, 35, 37, 37,
            39, 40, 41, 47, 50, 56, 59, 67, 70, 75, 79, 83, 89, 95, 96, 97, 98, 98, 99,
        ],
        [
            6, 9, 11, 12, 14, 15, 15, 17, 18, 20, 21, 22, 23, 25, 26, 28, 29, 31, 32, 35, 38, 38,
            39, 43, 45, 49, 52, 56, 59, 67, 71, 75, 79, 87, 93, 95, 97, 97, 98, 98, 99,
        ],
        [
            7, 10, 11, 13, 14, 15, 15, 17, 18, 20, 22, 22, 24, 26, 27, 29, 30, 32, 32, 36, 38, 39,
            40, 45, 48, 51, 53, 57, 60, 67, 71, 76, 80, 88, 93, 96, 97, 97, 98, 98, 99,
        ],
        [
            7, 10, 11, 13, 14, 15, 16, 17, 18, 20, 22, 23, 24, 26, 27, 29, 30, 32, 32, 36, 39, 40,
            41, 45, 48, 51, 53, 57, 61, 67, 72, 76, 82, 90, 96, 96, 97, 97, 98, 98, 99,
        ],
        [
            7, 10, 11, 13, 15, 15, 16, 17, 19, 20, 22, 24, 25, 26, 27, 29, 31, 32, 33, 37, 39, 42,
            43, 46, 48, 51, 53, 58, 61, 68, 72, 78, 82, 92, 96, 96, 97, 97, 98, 98, 99,
        ],
        [
            7, 10, 11, 13, 15, 15, 16, 18, 20, 20, 22, 24, 25, 27, 28, 31, 31, 32, 33, 37, 39, 43,
            45, 47, 49, 52, 53, 60, 63, 69, 73, 80, 84, 92, 96, 97, 97, 98, 98, 98, 99,
        ],
        [
            7, 10, 12, 13, 15, 16, 17, 19, 20, 21, 23, 24, 25, 27, 29, 31, 32, 32, 34, 38, 40, 43,
            45, 48, 50, 52, 54, 60, 64, 70, 74, 82, 84, 93, 97, 97, 98, 98, 98, 98, 99,
        ],
        [
            7, 10, 12, 13, 15, 16, 18, 19, 20, 22, 23, 24, 25, 28, 29, 31, 32, 34, 36, 38, 40, 43,
            46, 49, 50, 53, 54, 61, 65, 71, 74, 83, 84, 94, 97, 97, 98, 98, 98, 98, 99,
        ],
        [
            7, 10, 12, 14, 15, 16, 18, 19, 20, 22, 23, 24, 25, 28, 29, 31, 32, 35, 37, 39, 42, 44,
            46, 49, 51, 53, 55, 62, 66, 73, 76, 83, 85, 94, 97, 97, 98, 98, 98, 98, 99,
        ],
        [
            7, 10, 12, 14, 15, 16, 18, 20, 21, 22, 23, 24, 25, 28, 30, 31, 32, 36, 38, 41, 43, 45,
            46, 50, 52, 54, 55, 63, 67, 74, 79, 83, 85, 94, 97, 97, 98, 98, 98, 98, 99,
        ],
        [
            7, 10, 12, 14, 15, 16, 18, 20, 21, 23, 24, 25, 26, 29, 30, 31, 33, 36, 38, 42, 45, 45,
            46, 51, 53, 54, 56, 64, 68, 76, 80, 84, 87, 94, 97, 97, 98, 98, 98, 98, 99,
        ],
        [
            7, 10, 13, 14, 15, 17, 18, 20, 21, 24, 25, 27, 28, 29, 31, 32, 33, 36, 39, 43, 45, 46,
            47, 51, 54, 58, 61, 65, 68, 76, 80, 84, 87, 95, 97, 97, 98, 98, 98, 99, 99,
        ],
        [
            7, 10, 13, 14, 15, 17, 18, 20, 21, 24, 26, 28, 29, 29, 32, 32, 34, 37, 39, 43, 46, 47,
            48, 51, 55, 61, 64, 66, 68, 76, 80, 85, 89, 95, 97, 97, 98, 98, 99, 99, 99,
        ],
        [
            7, 10, 13, 14, 16, 17, 18, 20, 21, 24, 26, 28, 29, 31, 32, 33, 34, 37, 39, 43, 46, 47,
            48, 52, 55, 61, 64, 66, 68, 77, 81, 86, 91, 95, 97, 97, 98, 98, 99, 99, 99,
        ],
        [
            7, 10, 13, 14, 16, 17, 18, 20, 22, 24, 26, 28, 29, 31, 33, 34, 35, 38, 39, 44, 46, 48,
            49, 53, 55, 61, 64, 67, 69, 78, 83, 86, 93, 96, 97, 98, 98, 98, 99, 99, 100,
        ],
        [
            7, 11, 13, 14, 16, 18, 19, 21, 22, 25, 27, 28, 30, 32, 33, 34, 35, 38, 39, 44, 47, 48,
            50, 54, 57, 61, 64, 68, 70, 78, 83, 89, 94, 96, 97, 98, 98, 98, 99, 99, 100,
        ],
        [
            7, 11, 14, 15, 16, 18, 19, 21, 22, 25, 27, 29, 30, 32, 33, 35, 36, 38, 40, 44, 47, 49,
            50, 55, 58, 62, 64, 68, 70, 79, 85, 92, 94, 96, 97, 98, 98, 98, 99, 99, 100,
        ],
        [
            7, 11, 14, 15, 16, 19, 19, 21, 22, 25, 28, 29, 30, 32, 33, 37, 39, 39, 40, 45, 48, 49,
            50, 55, 58, 62, 64, 68, 71, 82, 86, 92, 94, 97, 97, 98, 98, 98, 99, 99, 100,
        ],
        [
            8, 11, 14, 15, 17, 19, 19, 21, 22, 26, 28, 29, 30, 33, 34, 38, 39, 40, 40, 45, 48, 50,
            51, 56, 59, 63, 65, 69, 72, 84, 88, 92, 96, 97, 98, 98, 98, 98, 99, 99, 100,
        ],
        [
            8, 11, 14, 15, 17, 19, 19, 21, 23, 26, 28, 30, 31, 33, 35, 39, 39, 40, 40, 45, 48, 50,
            51, 57, 59, 64, 65, 73, 76, 85, 90, 94, 96, 97, 98, 98, 98, 98, 99, 99, 100,
        ],
        [
            8, 11, 14, 15, 17, 19, 20, 21, 23, 26, 28, 30, 31, 34, 35, 39, 40, 40, 41, 46, 48, 51,
            53, 57, 60, 65, 67, 74, 77, 85, 90, 95, 96, 97, 98, 98, 98, 98, 99, 99, 100,
        ],
        [
            8, 11, 14, 15, 17, 19, 20, 22, 23, 26, 28, 30, 31, 34, 35, 39, 41, 41, 42, 46, 48, 52,
            53, 58, 61, 65, 67, 75, 79, 86, 90, 95, 96, 97, 98, 98, 99, 99, 99, 99, 100,
        ],
        [
            8, 11, 14, 16, 18, 19, 20, 22, 23, 27, 28, 30, 31, 34, 36, 39, 41, 41, 42, 46, 49, 52,
            55, 60, 63, 66, 69, 76, 79, 87, 90, 95, 97, 97, 98, 98, 99, 99, 99, 99, 100,
        ],
        [
            8, 11, 14, 16, 18, 19, 21, 22, 23, 27, 29, 30, 32, 34, 36, 39, 41, 41, 42, 46, 49, 53,
            55, 60, 63, 67, 69, 77, 80, 88, 92, 96, 97, 97, 98, 98, 99, 99, 99, 99, 100,
        ],
        [
            8, 11, 14, 16, 18, 20, 21, 22, 23, 27, 29, 32, 33, 34, 37, 40, 41, 42, 43, 47, 49, 53,
            55, 61, 64, 68, 72, 77, 82, 89, 92, 96, 97, 97, 98, 98, 99, 99, 99, 99, 100,
        ],
        [
            8, 11, 14, 16, 18, 20, 21, 22, 23, 27, 29, 32, 34, 37, 38, 40, 41, 42, 43, 47, 49, 54,
            56, 62, 64, 70, 72, 77, 82, 89, 93, 96, 97, 98, 98, 98, 99, 99, 100, 100, 100,
        ],
        [
            8, 11, 15, 16, 18, 21, 22, 23, 24, 27, 29, 32, 34, 37, 38, 40, 41, 43, 44, 47, 49, 54,
            57, 62, 65, 71, 74, 79, 83, 90, 93, 96, 98, 98, 98, 99, 99, 99, 100, 100, 100,
        ],
        [
            8, 11, 15, 16, 18, 21, 22, 24, 25, 27, 29, 32, 34, 37, 38, 41, 42, 44, 45, 48, 50, 54,
            58, 63, 65, 71, 74, 80, 84, 90, 93, 96, 98, 98, 98, 99, 99, 99, 100, 100, 100,
        ],
        [
            8, 11, 15, 16, 18, 21, 22, 25, 27, 27, 30, 32, 34, 37, 39, 41, 42, 46, 47, 48, 50, 55,
            58, 63, 66, 72, 75, 81, 84, 91, 94, 97, 98, 98, 98, 99, 99, 99, 100, 100, 100,
        ],
        [
            8, 12, 15, 17, 19, 21, 22, 25, 27, 29, 31, 33, 35, 37, 39, 42, 43, 46, 48, 48, 51, 56,
            60, 64, 66, 72, 76, 82, 85, 91, 94, 97, 98, 98, 98, 99, 99, 99, 100, 100, 100,
        ],
        [
            8, 12, 15, 17, 19, 21, 22, 26, 28, 30, 31, 33, 35, 38, 41, 42, 44, 46, 49, 50, 52, 57,
            61, 64, 67, 72, 77, 84, 87, 91, 95, 97, 98, 98, 99, 99, 99, 99, 100, 100, 100,
        ],
        [
            8, 12, 15, 17, 19, 21, 22, 26, 28, 30, 31, 33, 35, 38, 41, 42, 44, 46, 49, 53, 55, 59,
            62, 65, 67, 72, 77, 85, 88, 92, 95, 97, 98, 98, 99, 99, 99, 99, 100, 100, 100,
        ],
        [
            8, 12, 15, 17, 19, 21, 22, 26, 28, 31, 32, 34, 35, 39, 41, 42, 45, 47, 49, 53, 55, 59,
            62, 66, 68, 73, 77, 85, 88, 92, 95, 97, 98, 98, 99, 99, 99, 99, 100, 100, 100,
        ],
        [
            9, 12, 15, 17, 20, 21, 23, 26, 28, 31, 32, 34, 35, 39, 41, 43, 45, 47, 49, 54, 56, 60,
            62, 66, 68, 78, 82, 87, 90, 93, 95, 97, 98, 98, 99, 99, 100, 100, 100, 100, 100,
        ],
        [
            9, 12, 15, 17, 20, 22, 23, 26, 28, 31, 33, 34, 35, 39, 41, 43, 45, 48, 50, 54, 56, 60,
            62, 67, 70, 79, 83, 88, 90, 94, 96, 97, 98, 98, 99, 99, 100, 100, 100, 100, 100,
        ],
        [
            9, 12, 15, 18, 21, 22, 23, 26, 28, 31, 33, 34, 35, 40, 41, 44, 45, 48, 50, 54, 56, 60,
            62, 68, 70, 79, 83, 88, 91, 94, 96, 98, 98, 99, 99, 99, 100, 100, 100, 100, 100,
        ],
        [
            9, 12, 15, 18, 21, 22, 24, 26, 28, 31, 33, 34, 36, 40, 42, 44, 45, 49, 50, 54, 57, 61,
            63, 69, 71, 79, 84, 88, 92, 95, 97, 98, 98, 99, 99, 99, 100, 100, 100, 100, 100,
        ],
        [
            9, 13, 15, 18, 21, 22, 24, 26, 28, 31, 33, 34, 36, 40, 42, 44, 46, 50, 52, 55, 57, 61,
            64, 69, 71, 80, 85, 89, 92, 95, 97, 98, 99, 99, 99, 99, 100, 100, 100, 100, 100,
        ],
        [
            9, 13, 15, 18, 21, 22, 24, 26, 28, 31, 33, 35, 36, 40, 42, 45, 47, 50, 52, 56, 58, 61,
            64, 69, 72, 80, 85, 89, 92, 95, 98, 98, 99, 99, 99, 99, 100, 100, 100, 100, 100,
        ],
        [
            9, 13, 15, 18, 21, 22, 24, 27, 28, 31, 33, 35, 37, 40, 42, 45, 47, 51, 52, 56, 59, 62,
            64, 70, 73, 80, 85, 90, 93, 96, 98, 98, 99, 99, 99, 99, 100, 100, 100, 100, 100,
        ],
        [
            9, 13, 15, 18, 21, 23, 25, 27, 29, 31, 33, 36, 38, 40, 42, 45, 47, 51, 53, 56, 59, 63,
            65, 70, 73, 81, 85, 91, 94, 96, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            9, 13, 16, 19, 21, 23, 25, 27, 29, 32, 33, 36, 38, 40, 43, 46, 48, 51, 53, 56, 59, 64,
            66, 70, 74, 81, 85, 91, 94, 96, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            9, 13, 16, 19, 21, 23, 25, 27, 30, 32, 33, 36, 38, 40, 43, 46, 48, 51, 54, 56, 60, 64,
            67, 71, 74, 81, 86, 93, 94, 97, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            9, 13, 16, 19, 21, 23, 26, 28, 30, 32, 34, 36, 38, 41, 43, 47, 49, 51, 54, 58, 60, 64,
            67, 72, 76, 82, 86, 93, 94, 97, 98, 99, 99, 99, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            9, 13, 16, 19, 21, 23, 26, 28, 30, 32, 34, 36, 38, 41, 44, 47, 50, 52, 54, 58, 61, 65,
            67, 73, 76, 83, 87, 93, 95, 97, 98, 99, 99, 99, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            9, 14, 16, 19, 21, 23, 26, 28, 30, 32, 34, 37, 39, 41, 44, 47, 50, 52, 55, 59, 61, 65,
            67, 74, 77, 86, 89, 93, 96, 97, 98, 99, 99, 99, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            9, 14, 16, 19, 21, 24, 26, 29, 31, 32, 34, 37, 39, 42, 45, 48, 51, 53, 55, 59, 62, 65,
            68, 74, 77, 87, 91, 93, 96, 97, 98, 99, 99, 99, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            9, 14, 16, 19, 21, 24, 26, 29, 31, 33, 35, 38, 39, 43, 45, 49, 51, 54, 56, 59, 62, 66,
            69, 74, 77, 87, 91, 95, 96, 97, 98, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 14, 16, 19, 22, 24, 27, 30, 31, 34, 36, 38, 39, 44, 47, 50, 52, 55, 58, 60, 62, 67,
            70, 74, 78, 88, 91, 95, 96, 97, 98, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 14, 17, 20, 22, 24, 27, 30, 31, 35, 36, 38, 39, 45, 47, 50, 52, 56, 58, 62, 63, 68,
            70, 75, 79, 88, 91, 95, 97, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 14, 17, 20, 22, 24, 27, 30, 32, 35, 36, 38, 40, 45, 48, 50, 52, 56, 59, 62, 64, 68,
            71, 76, 79, 88, 92, 95, 97, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 14, 17, 20, 22, 24, 27, 31, 33, 35, 36, 40, 41, 45, 48, 51, 52, 56, 59, 62, 65, 69,
            71, 77, 81, 88, 92, 95, 97, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 15, 17, 20, 22, 24, 27, 31, 33, 35, 37, 40, 41, 46, 48, 51, 53, 57, 60, 64, 67, 70,
            72, 78, 81, 88, 92, 95, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 15, 17, 20, 22, 25, 27, 31, 33, 36, 39, 41, 42, 47, 48, 52, 53, 57, 60, 64, 67, 70,
            72, 79, 83, 89, 93, 96, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 15, 17, 20, 22, 25, 28, 31, 33, 36, 39, 41, 43, 47, 48, 52, 53, 57, 60, 65, 69, 72,
            74, 80, 83, 89, 94, 96, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 15, 17, 20, 23, 25, 28, 31, 33, 37, 39, 41, 43, 47, 49, 52, 54, 58, 60, 66, 69, 72,
            74, 80, 84, 89, 94, 96, 98, 98, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 15, 17, 20, 24, 25, 28, 31, 33, 37, 39, 41, 43, 47, 49, 52, 54, 58, 60, 66, 69, 72,
            74, 81, 85, 89, 94, 96, 98, 98, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 15, 18, 20, 24, 25, 28, 31, 33, 37, 39, 42, 44, 48, 49, 53, 55, 59, 61, 66, 69, 73,
            76, 82, 85, 89, 95, 96, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 15, 18, 20, 24, 25, 28, 32, 34, 37, 39, 42, 44, 48, 49, 53, 55, 60, 61, 67, 69, 75,
            78, 82, 85, 91, 95, 97, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 15, 18, 21, 24, 25, 29, 32, 34, 37, 39, 42, 45, 48, 50, 53, 55, 60, 62, 67, 69, 75,
            78, 82, 85, 91, 95, 97, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 15, 19, 21, 24, 25, 29, 32, 34, 37, 39, 43, 45, 48, 51, 53, 55, 60, 63, 68, 69, 75,
            78, 83, 85, 93, 95, 97, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 15, 19, 21, 24, 26, 30, 32, 34, 37, 39, 43, 45, 48, 51, 54, 56, 61, 64, 68, 70, 76,
            79, 86, 89, 95, 96, 97, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 16, 19, 21, 25, 26, 30, 33, 34, 38, 40, 43, 45, 48, 51, 54, 57, 62, 66, 69, 70, 76,
            79, 86, 91, 95, 96, 97, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
        [
            10, 16, 19, 21, 25, 26, 31, 33, 35, 38, 40, 43, 45, 49, 51, 55, 57, 63, 66, 70, 72, 77,
            81, 87, 91, 95, 97, 98, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100,
        ],
        [
            10, 16, 19, 21, 25, 26, 31, 33, 35, 38, 40, 43, 46, 49, 51, 55, 59, 63, 66, 70, 73, 78,
            81, 88, 91, 95, 97, 98, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100,
        ],
        [
            10, 16, 20, 21, 25, 26, 32, 34, 35, 39, 40, 43, 46, 49, 51, 55, 59, 63, 67, 70, 75, 79,
            82, 89, 93, 95, 98, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100,
        ],
        [
            10, 16, 20, 21, 25, 27, 32, 34, 35, 39, 41, 44, 47, 49, 52, 56, 60, 64, 67, 71, 76, 80,
            83, 90, 94, 97, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100,
        ],
        [
            10, 16, 20, 22, 26, 27, 32, 35, 37, 39, 41, 46, 49, 49, 52, 60, 64, 65, 69, 72, 76, 81,
            84, 90, 94, 97, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100,
        ],
        [
            10, 17, 21, 23, 26, 29, 32, 36, 38, 39, 41, 47, 51, 51, 54, 62, 66, 68, 69, 74, 77, 82,
            85, 91, 96, 97, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100,
        ],
    ];
}
pub mod solver {
    use crate::graph::{calc_graph_similarity, Graph};

    pub fn solve(graphs: &Vec<Graph>, h: &Graph, m: usize, eps: f64) -> usize {
        let mut best_graph_index = 0;
        let mut min_score = 1e10;

        for i in 0..graphs.len() {
            let score = calc_graph_similarity(&h, &graphs[i], m, eps);
            if score < min_score {
                min_score = score;
                best_graph_index = i;
            }
        }

        best_graph_index
    }
}
pub mod util {
    #[allow(unused_features)]

    pub mod rnd {
        #[allow(unused)]
        static mut S: usize = 88172645463325252;

        #[allow(unused)]
        #[inline]
        pub fn next() -> usize {
            unsafe {
                S = S ^ S << 7;
                S = S ^ S >> 9;
                S
            }
        }

        #[allow(unused)]
        #[inline]
        pub fn nextf() -> f64 {
            (next() & 4294967295) as f64 / 4294967296.
        }

        #[allow(unused)]
        #[inline]
        pub fn gen_range(low: usize, high: usize) -> usize {
            (next() % (high - low)) + low
        }
    }

    pub mod time {
        static mut START: f64 = -1.;
        #[allow(unused)]
        pub fn start_clock() {
            let _ = elapsed_seconds();
        }

        #[allow(unused)]
        #[inline]
        pub fn elapsed_seconds() -> f64 {
            let t = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            unsafe {
                if START < 0. {
                    START = t;
                }
                t - START
            }
        }
    }

    pub fn generate_shuffled_permutation(n: usize) -> Vec<usize> {
        let mut p = vec![0; n];
        for i in 0..n {
            p[i] = i;
        }
        // シャッフル
        for i in 0..n {
            let j = rnd::gen_range(0, n);
            p.swap(i, j);
        }
        p
    }
}

use std::io;
use std::io::Write;

use crate::gen::*;
use crate::graph::*;
use crate::param::BEST_N;
use crate::solver::solve;
use crate::util::time;

fn read_input(stdin: &io::Stdin) -> (usize, f64) {
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
    let mut v = vec![];
    for e in user_input.trim().split(" ") {
        v.push(e.to_string());
    }
    (v[0].parse().unwrap(), v[1].parse().unwrap())
}

fn read_graph_input(stdin: &io::Stdin) -> String {
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
    user_input
}

fn main() {
    time::start_clock();

    const QUERY_COUNT: usize = 100;
    const CONSTRUCT_TIME_LIMIT: f64 = 4.7;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let flush = || stdout.lock().flush().unwrap();

    let (m, eps) = read_input(&stdin);

    let n = BEST_N[m - 10][(eps * 100.).round() as usize];

    eprintln!("M = {}, eps = {}, N = {}", m, eps, n);

    // M, epsに対応するグラフを出力する
    let graphs = create_optimal_graphs(n, m, eps, CONSTRUCT_TIME_LIMIT);

    println!("{}", n);

    for graph in &graphs {
        println!("{}", graph.to_raw_format());
    }
    flush();

    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());

    // 各クエリを処理する
    for _ in 0..QUERY_COUNT {
        let raw_h = read_graph_input(&stdin);

        // hとGとの類似度を求め、類似度が最大のGを出力する
        let mut h = Graph::from_raw_format(n, &raw_h);
        // h.degreesをシミュレーション後の次数列とみなす
        h.simulated_degrees = h.degrees.clone();
        h.simulated_degrees
            .sort_by(|a, b| a.partial_cmp(b).unwrap());
        h.simulated_squares = calc_simulated_square(&h);

        let best_graph_index = solve(&graphs, &h, m, eps);
        println!("{}", best_graph_index);
        flush();
    }

    eprintln!("elapsed seconds: {:.4}", time::elapsed_seconds());
}
