use super::util::*;

#[derive(Debug)]
pub struct SegTree<T, F>
where
    F: Fn(T, T) -> T,
{
    size: usize,
    nil: T,
    operation: F,
    store: Vec<T>,
}

impl<T: Clone, F> SegTree<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(size: usize, nil: T, operation: F) -> Self {
        Self {
            size,
            nil: nil.clone(),
            operation,
            store: vec![nil; 2 * size],
        }
    }

    pub fn from_iter<U: IntoIterator<Item = T>>(
        iter: U,
        nil: T,
        operation: F,
    ) -> Self {
        let source: Vec<_> = iter.into_iter().collect();
        let mut result = Self::new(source.len(), nil, operation);
        result.build_from_vec(&source, 0, 0, source.len() - 1);
        result
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn set(&mut self, pos: usize, value: T) {
        self.set_impl(pos, value, 0, 0, self.size - 1)
    }

    pub fn update(&mut self, pos: usize, value: T) {
        self.update_impl(pos, value, 0, 0, self.size - 1)
    }

    pub fn query(&self, left: usize, right: usize) -> T {
        self.query_impl(left, right, 0, 0, self.size - 1)
    }

    fn build_from_vec(
        &mut self,
        source: &Vec<T>,
        idx: usize,
        l: usize,
        r: usize,
    ) {
        if l == r {
            self.store[idx] = source[l].clone();
            return;
        }
        let (lc, rc, m) = compute_indices(idx, l, r);
        self.build_from_vec(source, lc, l, m);
        self.build_from_vec(source, rc, m + 1, r);
        self.store[idx] =
            (self.operation)(self.store[lc].clone(), self.store[rc].clone());
    }

    fn set_impl(&mut self, u: usize, w: T, idx: usize, l: usize, r: usize) {
        if u > r || u < l {
            return;
        }
        if u == l && u == r {
            self.store[idx] = w;
            return;
        }
        let (lc, rc, m) = compute_indices(idx, l, r);
        if u <= m {
            self.set_impl(u, w, lc, l, m);
        } else {
            self.set_impl(u, w, rc, m + 1, r);
        }
        self.store[idx] =
            (self.operation)(self.store[lc].clone(), self.store[rc].clone());
    }

    fn update_impl(&mut self, u: usize, w: T, idx: usize, l: usize, r: usize) {
        if u > r || u < l {
            return;
        }
        if u == l && u == r {
            self.store[idx] = (self.operation)(self.store[idx].clone(), w);
            return;
        }
        let (lc, rc, m) = compute_indices(idx, l, r);
        if u <= m {
            self.update_impl(u, w, lc, l, m);
        } else {
            self.update_impl(u, w, rc, m + 1, r);
        }
        self.store[idx] =
            (self.operation)(self.store[lc].clone(), self.store[rc].clone());
    }

    fn query_impl(
        &self,
        u: usize,
        v: usize,
        idx: usize,
        l: usize,
        r: usize,
    ) -> T {
        if u > r || v < l {
            return self.nil.clone();
        }
        if u <= l && v >= r {
            return self.store[idx].clone();
        }
        let (lc, rc, m) = compute_indices(idx, l, r);
        (self.operation)(
            self.query_impl(u, v, lc, l, m),
            self.query_impl(u, v, rc, m + 1, r),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        let data = [69, 420, 123, 277, 1337];

        let mut with_new = SegTree::new(5, 0, |a, b| a + b);
        for (i, &val) in data.iter().enumerate() {
            with_new.set(i, val);
        }

        let with_from = SegTree::from_iter(data, 0, |a, b| a + b);

        assert_eq!(with_new.store, with_from.store);
    }

    #[test]
    fn test_query() {
        let data = [69, 420, 123, 277, 1337, 1234, 5785, 156, 278];
        let seg_tree = SegTree::from_iter(data, 0, |a, b| a + b);

        for i in 0..data.len() {
            for j in i..data.len() {
                let sum: i32 = data[i..=j].iter().sum();
                assert_eq!(sum, seg_tree.query(i, j));
            }
        }
    }

    #[test]
    fn test_update() {
        let mut data = [69, 420, 123, 277, 1337, 1234, 5785, 156, 278];
        let updates = [(2, 7), (3, 9), (4, 2187), (7, 123), (2, 188), (0, 17)];

        let mut seg_tree = SegTree::from_iter(data, 0, |a, b| a + b);

        for (idx, val) in updates {
            data[idx] += val;
            seg_tree.update(idx, val);

            for i in 0..data.len() {
                for j in i..data.len() {
                    let sum: i32 = data[i..=j].iter().sum();
                    assert_eq!(sum, seg_tree.query(i, j));
                }
            }
        }
    }
}
