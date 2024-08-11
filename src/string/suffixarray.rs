use std::iter::once;

fn fill_sa_rank(
    order: &[usize],
    rank: &mut [usize],
    is_same: impl Fn(usize, usize) -> bool,
) {
    let n = order.len();
    let mut count = 0;
    let mut last = n;
    for &cur in order {
        if last != n && !is_same(cur, last) {
            count += 1;
        }
        rank[cur] = count;
        last = cur;
    }
}

fn update_sa_order(
    order: &mut [usize],
    buffer: &mut [usize],
    counts: &mut [usize],
    key_fn: impl Fn(usize) -> usize,
    num_keys: usize,
) {
    for i in 0..num_keys {
        counts[i] = 0;
    }
    for i in 0..order.len() {
        buffer[i] = order[i];
        counts[key_fn(i)] += 1;
    }
    for i in 1..num_keys {
        counts[i] += counts[i - 1];
    }
    for i in (0..order.len()).rev() {
        let key = key_fn(buffer[i]);
        counts[key] -= 1;
        order[counts[key]] = buffer[i];
    }
}

pub fn get_ordered_cyclic_shifts<T: Ord>(
    iter: impl Iterator<Item = T>,
) -> Vec<usize> {
    let s: Vec<_> = iter.collect();

    let n = s.len();
    let mut order: Vec<_> = (0..n).collect();
    order.sort_unstable_by_key(|&x| &s[x]);

    let mut buffer = vec![0; n];
    let mut rank = vec![0; n];
    let mut counts = vec![0; n];
    let mut next_idx = vec![0; n];

    fill_sa_rank(&order, &mut rank, |a, b| s[a] == s[b]);

    let mut k = 1;
    while k < n {
        for i in 0..n - k {
            next_idx[i] = i + k;
        }
        for i in n - k..n {
            next_idx[i] = i + k - n;
        }

        let num_keys = *rank.iter().max().unwrap() + 1;
        update_sa_order(
            &mut order,
            &mut buffer,
            &mut counts,
            |x| rank[next_idx[x]],
            num_keys,
        );
        update_sa_order(
            &mut order,
            &mut buffer,
            &mut counts,
            |x| rank[x],
            num_keys,
        );

        for i in 0..n {
            buffer[i] = rank[i];
        }
        fill_sa_rank(&order, &mut rank, |a, b| {
            buffer[next_idx[a]] == buffer[next_idx[b]] && buffer[a] == buffer[b]
        });

        k *= 2;
    }

    return order;
}

pub fn get_suffix_array(s: &str) -> Vec<usize> {
    let mut res =
        get_ordered_cyclic_shifts(s.as_bytes().iter().chain(once(&b'$')));
    res.retain(|&x| x != s.len());
    return res;
}

pub fn get_lcp_array(s: &str, suffix_array: &[usize]) -> Vec<usize> {
    let n = s.len();
    let s = s.as_bytes();
    let mut rank = vec![0; n];
    for i in 0..n {
        rank[suffix_array[i]] = i;
    }
    let mut k = 0;
    let mut lcp = vec![0; n];
    for i in 0..n {
        if rank[i] == n - 1 {
            k = 0;
            continue;
        }
        let j = suffix_array[rank[i] + 1];
        while i + k < n && j + k < n && s[i + k] == s[j + k] {
            k += 1;
        }
        lcp[rank[i]] = k;
        if k > 0 {
            k -= 1;
        }
    }
    return lcp;
}

#[cfg(test)]
mod test {
    use super::*;
    const STRING: &str = "mississippi";

    #[test]
    fn test_mississippi() {
        assert_eq!(
            get_suffix_array(STRING),
            [10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]
        );
    }

    #[test]
    fn test_lcp() {
        let sa = get_suffix_array(STRING);
        let lcp = get_lcp_array(STRING, &sa);
        assert_eq!(lcp, [1, 1, 4, 0, 0, 1, 0, 2, 1, 3, 0]);
    }
}
