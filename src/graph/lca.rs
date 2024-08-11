use crate::ds::sparsetable::SparseTable;
use crate::misc::rec_lambda::*;
use std::cmp::min;

pub struct LCA {
    t_in: Vec<usize>,
    t_out: Vec<usize>,
    level: Vec<usize>,
    sparse: SparseTable<
        (usize, usize),
        fn((usize, usize), (usize, usize)) -> (usize, usize),
    >,
}

impl LCA {
    pub fn new(
        num_vertices: usize,
        adj_list: Vec<Vec<usize>>,
        root: usize,
    ) -> Self {
        let mut t_in = vec![0; num_vertices];
        let mut t_out = vec![0; num_vertices];
        let mut level = vec![0; num_vertices];
        let mut order = Vec::new();

        let mut dfs = rec_lambda! {
            [adj_list: Vec<Vec<usize>>]
            [
                t_in: Vec<usize>, t_out: Vec<usize>,
                level: Vec<usize>, order: Vec<usize>
            ]
            (pos: usize, last: usize, depth: usize) -> () {
                t_in[pos] = order.len();
                t_out[pos] = order.len();
                level[pos] = depth;
                order.push(pos);

                for &child in &adj_list[pos] {
                    if child == last {
                        continue;
                    }
                    recurse!(child, pos, depth + 1);

                    t_out[pos] = order.len();
                    order.push(pos);
                }
            }
        };
        dfs(root, num_vertices, 0);

        let level_iter: Vec<_> = order.iter().map(|&x| level[x]).collect();
        Self {
            sparse: SparseTable::from_iter(
                level_iter.into_iter().zip(order),
                (0, 0),
                min::<(usize, usize)>,
            ),
            t_in,
            t_out,
            level,
        }
    }

    pub fn get_lca_distance(&self, u: usize, v: usize) -> (usize, usize) {
        let l = self.t_in[u];
        let r = self.t_out[v];
        let (lca_level, lca) = if l <= r {
            self.sparse.query(l, r)
        } else {
            self.sparse.query(r, l)
        };
        return (lca, self.level[u] + self.level[v] - lca_level * 2);
    }

    pub fn get_lca(&self, u: usize, v: usize) -> usize {
        self.get_lca_distance(u, v).0
    }

    pub fn get_distance(&self, u: usize, v: usize) -> usize {
        self.get_lca_distance(u, v).1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::util::edge_list_to_adj_list;

    #[test]
    fn test_all() {
        const N: usize = 8;
        const EDGES: [(usize, usize); 7] =
            [(0, 1), (0, 2), (2, 7), (3, 7), (4, 2), (5, 3), (0, 6)];

        let lca = LCA::new(N, edge_list_to_adj_list(N, EDGES, false), 0);

        assert_eq!(lca.get_lca(1, 2), 0);
        assert_eq!(lca.get_lca(6, 4), 0);
        assert_eq!(lca.get_lca(1, 5), 0);
        assert_eq!(lca.get_lca(3, 5), 3);
        assert_eq!(lca.get_lca(7, 4), 2);
        assert_eq!(lca.get_lca(2, 2), 2);
        assert_eq!(lca.get_lca(7, 5), 7);

        assert_eq!(lca.get_distance(6, 4), 3);
        assert_eq!(lca.get_distance(4, 5), 4);
        assert_eq!(lca.get_distance(1, 0), 1);
        assert_eq!(lca.get_distance(1, 3), 4);
    }
}
