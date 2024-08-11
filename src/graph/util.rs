pub fn edge_list_to_adj_list(
    num_vertices: usize,
    edges: impl IntoIterator<Item = (usize, usize)>,
    directed: bool,
) -> Vec<Vec<usize>> {
    let mut res = vec![vec![]; num_vertices];
    for (u, v) in edges {
        if !directed {
            res[v].push(u);
        }
        res[u].push(v);
    }
    return res;
}

pub fn edge_list_to_adj_list_weighted<W: Clone>(
    num_vertices: usize,
    edges: impl IntoIterator<Item = (usize, usize, W)>,
    directed: bool,
) -> Vec<Vec<(usize, W)>> {
    let mut res = vec![vec![]; num_vertices];
    for (u, v, w) in edges {
        if !directed {
            res[v].push((u, w.clone()));
        }
        res[u].push((v, w));
    }
    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    const N: usize = 8;
    const EDGES: [(usize, usize, i32); 7] = [
        (0, 1, 2),
        (0, 2, 1),
        (2, 7, 17),
        (3, 7, 2),
        (4, 2, 9),
        (5, 3, 13),
        (0, 6, 2),
    ];

    #[test]
    fn test_unweighted() {
        let edges = EDGES.iter().map(|&(u, v, _)| (u, v));
        let mut adj_list = edge_list_to_adj_list(N, edges, false);
        let mut expected = vec![
            vec![1, 2, 6],
            vec![0],
            vec![0, 4, 7],
            vec![5, 7],
            vec![2],
            vec![3],
            vec![0],
            vec![2, 3],
        ];
        assert_eq!(adj_list.len(), expected.len());
        for i in 0..adj_list.len() {
            adj_list[i].sort_unstable();
            expected[i].sort_unstable();
            assert_eq!(adj_list[i], expected[i]);
        }
    }

    #[test]
    fn test_weighted() {
        let mut adj_list = edge_list_to_adj_list_weighted(N, EDGES, false);
        let mut expected = vec![
            vec![(1, 2), (2, 1), (6, 2)],
            vec![(0, 2)],
            vec![(0, 1), (4, 9), (7, 17)],
            vec![(5, 13), (7, 2)],
            vec![(2, 9)],
            vec![(3, 13)],
            vec![(0, 2)],
            vec![(2, 17), (3, 2)],
        ];
        assert_eq!(adj_list.len(), expected.len());
        for i in 0..adj_list.len() {
            adj_list[i].sort_unstable();
            expected[i].sort_unstable();
            assert_eq!(adj_list[i], expected[i]);
        }
    }
}
