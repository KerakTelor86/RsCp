use std::cmp::min;
use std::collections::VecDeque;
use std::iter::Peekable;
use std::ops::Range;

pub fn get_max_flow(
    source: usize,
    sink: usize,
    adj_list: &Vec<Vec<(usize, i64)>>,
    directed: bool,
    use_scaling: bool,
) -> i64 {
    let n = adj_list.len();

    assert!(source < n);
    assert!(sink < n);
    let mut adj_dinic = vec![vec![]; n];
    let mut edges = Vec::<Edge>::new();
    for (from, adj) in adj_list.into_iter().enumerate() {
        for &(to, cap) in adj {
            adj_dinic[from].push(edges.len());
            edges.push(Edge { to, cap, flow: 0 });
            adj_dinic[to].push(edges.len());
            edges.push(Edge {
                to: from,
                cap: if directed { 0 } else { cap },
                flow: 0,
            })
        }
    }

    let mut dinic = Dinic {
        n,
        adj_dinic,
        edges,
        lim: if use_scaling { 1_i64 << 62 } else { 1 },
        depth: vec![],
        next_idx: vec![],
    };

    dinic.solve(source, sink)
}

struct Edge {
    to: usize,
    cap: i64,
    flow: i64,
}

struct Dinic {
    n: usize,
    adj_dinic: Vec<Vec<usize>>,
    edges: Vec<Edge>,
    lim: i64,
    depth: Vec<usize>,
    next_idx: Vec<Peekable<Range<usize>>>,
}

impl Dinic {
    fn has_path(&mut self, from: usize, to: usize) -> bool {
        let mut q = VecDeque::new();
        q.push_back(from);
        self.depth = vec![usize::MAX; self.n];
        self.depth[from] = 0;
        while let Some(cur) = q.pop_front() {
            if cur == to {
                break;
            }
            for &next in &self.adj_dinic[cur] {
                let Edge { to, cap, flow } = &self.edges[next];
                let cur_flow = *cap - *flow;
                if self.depth[*to] == usize::MAX && cur_flow >= self.lim {
                    self.depth[*to] = self.depth[cur] + 1;
                    q.push_back(*to);
                }
            }
        }
        self.depth[to] != usize::MAX
    }

    fn get_flow(&mut self, cur: usize, target: usize, left: i64) -> i64 {
        if left == 0 || cur == target {
            return left;
        }
        while let Some(&adj_idx) = self.next_idx[cur].peek() {
            let edge_idx = self.adj_dinic[cur][adj_idx];
            let Edge { to, cap, flow } = &self.edges[edge_idx];
            if self.depth[*to] == self.depth[cur] + 1 {
                let add = self.get_flow(*to, target, min(left, *cap - *flow));
                if add > 0 {
                    self.edges[edge_idx].flow += add;
                    self.edges[edge_idx ^ 1].flow -= add;
                    return add;
                }
            }
            self.next_idx[cur].next();
        }
        0
    }

    fn solve(&mut self, source: usize, sink: usize) -> i64 {
        let mut res = 0;
        while self.lim > 0 {
            while self.has_path(source, sink) {
                self.next_idx = (0..self.n)
                    .map(|i| (0..self.adj_dinic[i].len()).peekable())
                    .collect();
                loop {
                    match self.get_flow(source, sink, i64::MAX) {
                        0 => break,
                        val => res += val,
                    }
                }
            }
            self.lim /= 2;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::graph::dinic::get_max_flow;
    use crate::graph::util::edge_list_to_adj_list_weighted;

    #[test]
    fn test_max_flow() {
        let edges = [(0, 1, 3), (1, 3, 2), (0, 2, 4), (2, 3, 5), (3, 0, 3)];
        let adj = edge_list_to_adj_list_weighted(4, edges, true);
        let max_flow = get_max_flow(0, 3, &adj, true, true);
        assert_eq!(max_flow, 6);
    }
}
