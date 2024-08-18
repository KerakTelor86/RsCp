use crate::misc::macros::rec_lambda;

pub fn get_sccs(adj_list: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = adj_list.len();
    let mut ans = vec![];

    let mut adj_list_rev = vec![vec![]; n];
    for (i, adj) in adj_list.iter().enumerate() {
        for &j in adj {
            adj_list_rev[j].push(i);
        }
    }

    let dfs = rec_lambda!(|rec: Self,
                           pos: usize,
                           vis: &mut Vec<bool>,
                           out: &mut Vec<usize>,
                           adj: &Vec<Vec<usize>>|
     -> () {
        for &i in &adj[pos] {
            if !vis[i] {
                vis[i] = true;
                rec(i, vis, out, adj);
            }
        }
        out.push(pos);
    });

    let mut vis = vec![false; n];
    let mut order = vec![];
    for i in 0..n {
        if !vis[i] {
            vis[i] = true;
            dfs(i, &mut vis, &mut order, &adj_list);
        }
    }

    vis.fill(false);
    while let Some(i) = order.pop() {
        if !vis[i] {
            vis[i] = true;
            let mut cur = vec![];
            dfs(i, &mut vis, &mut cur, &adj_list_rev);
            ans.push(cur);
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::misc::fluent::FluentIteratorOrd;

    #[test]
    fn test_scc() {
        let adj = vec![
            vec![1, 7],
            vec![1, 2],
            vec![1, 5],
            vec![2, 4],
            vec![9],
            vec![3, 6, 9],
            vec![2],
            vec![0, 6, 8],
            vec![6, 9],
            vec![4],
        ];

        let expected =
            vec![vec![0, 7], vec![1, 2, 3, 5, 6], vec![4, 9], vec![8]];
        let sccs: Vec<_> = get_sccs(&adj)
            .into_iter()
            .map(|x| x.into_iter().sorted().collect::<Vec<_>>())
            .sorted()
            .collect();
        assert_eq!(sccs, expected);
    }
}
