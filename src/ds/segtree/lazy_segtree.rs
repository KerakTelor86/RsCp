use super::util::*;
use crate::misc::range::RangeWrapper;

#[derive(Debug)]
pub struct LazySegTree<T: Clone, U: Clone + Eq, F, L, M>
where
    F: Fn(T, T) -> T,
    L: Fn(T, U, usize, usize) -> T,
    M: Fn(U, U) -> U,
{
    size: usize,
    nil_value: T,
    nil_lazy: U,
    op_value: F,
    apply: L,
    op_lazy: M,
    store: Vec<T>,
    lazy_store: Vec<U>,
}

impl<T: Clone, U: Clone + Eq, F, L, M> LazySegTree<T, U, F, L, M>
where
    F: Fn(T, T) -> T,
    L: Fn(T, U, usize, usize) -> T,
    M: Fn(U, U) -> U,
{
    pub fn new(
        size: usize,
        nil_value: T,
        nil_lazy: U,
        operation: F,
        apply_lazy: L,
        merge_lazy: M,
    ) -> Self {
        Self {
            size,
            nil_value: nil_value.clone(),
            nil_lazy: nil_lazy.clone(),
            op_value: operation,
            apply: apply_lazy,
            op_lazy: merge_lazy,
            store: vec![nil_value; 2 * size],
            lazy_store: vec![nil_lazy; 2 * size],
        }
    }

    pub fn from_iter<V: IntoIterator<Item = T>>(
        iter: V,
        nil_value: T,
        nil_lazy: U,
        operation: F,
        apply_lazy: L,
        merge_lazy: M,
    ) -> Self {
        let source: Vec<_> = iter.into_iter().collect();
        let mut result = Self::new(
            source.len(),
            nil_value,
            nil_lazy,
            operation,
            apply_lazy,
            merge_lazy,
        );
        result.build_from_vec(&source, 0, 0, source.len() - 1);
        result
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn set(&mut self, pos: usize, value: T) {
        self.set_impl(pos, value, 0, 0, self.size - 1)
    }

    pub fn update(&mut self, range: impl RangeWrapper<usize>, lazy: U) {
        let (left, right) = range.closed_bounds();
        self.update_impl(left, right, lazy, 0, 0, self.size - 1)
    }

    pub fn query(&mut self, range: impl RangeWrapper<usize>) -> T {
        let (left, right) = range.closed_bounds();
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
            (self.op_value)(self.store[lc].clone(), self.store[rc].clone());
    }

    fn propagate(&mut self, idx: usize, l: usize, r: usize) {
        if self.lazy_store[idx] == self.nil_lazy {
            return;
        }
        if l != r {
            let (lc, rc, _) = compute_indices(idx, l, r);
            for child in [lc, rc] {
                self.lazy_store[child] = (self.op_lazy)(
                    self.lazy_store[child].clone(),
                    self.lazy_store[idx].clone(),
                );
            }
        }
        self.store[idx] = (self.apply)(
            self.store[idx].clone(),
            self.lazy_store[idx].clone(),
            l,
            r,
        );
        self.lazy_store[idx] = self.nil_lazy.clone();
    }

    fn set_impl(&mut self, u: usize, w: T, idx: usize, l: usize, r: usize) {
        self.propagate(idx, l, r);
        if u > r || u < l {
            return;
        }
        if u == l && u == r {
            self.store[idx] = w;
            return;
        }
        let (lc, rc, m) = compute_indices(idx, l, r);
        self.set_impl(u, w.clone(), lc, l, m);
        self.set_impl(u, w, rc, m + 1, r);
        self.store[idx] =
            (self.op_value)(self.store[lc].clone(), self.store[rc].clone());
    }

    fn update_impl(
        &mut self,
        u: usize,
        v: usize,
        w: U,
        idx: usize,
        l: usize,
        r: usize,
    ) {
        self.propagate(idx, l, r);
        if u > r || v < l {
            return;
        }
        if u <= l && v >= r {
            self.lazy_store[idx] =
                (self.op_lazy)(self.lazy_store[idx].clone(), w);
            self.propagate(idx, l, r);
            return;
        }
        let (lc, rc, m) = compute_indices(idx, l, r);
        self.update_impl(u, v, w.clone(), lc, l, m);
        self.update_impl(u, v, w, rc, m + 1, r);
        self.store[idx] =
            (self.op_value)(self.store[lc].clone(), self.store[rc].clone());
    }

    fn query_impl(
        &mut self,
        u: usize,
        v: usize,
        idx: usize,
        l: usize,
        r: usize,
    ) -> T {
        self.propagate(idx, l, r);
        if u > r || v < l {
            return self.nil_value.clone();
        }
        if u <= l && v >= r {
            return self.store[idx].clone();
        }
        let (lc, rc, m) = compute_indices(idx, l, r);
        let lc_res = self.query_impl(u, v, lc, l, m);
        let rc_res = self.query_impl(u, v, rc, m + 1, r);
        (self.op_value)(lc_res, rc_res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        let data = [69, 420, 123, 277, 1337];

        let mut with_new = LazySegTree::new(
            5,
            0,
            0,
            |a, b| a + b,
            |val, lazy, l, r| val + lazy * (r - l + 1),
            |a, b| a + b,
        );
        for (i, &val) in data.iter().enumerate() {
            with_new.set(i, val);
        }

        let with_from = LazySegTree::from_iter(
            data,
            0,
            0,
            |a, b| a + b,
            |val, lazy, l, r| val + lazy * (r - l + 1),
            |a, b| a + b,
        );

        assert_eq!(with_new.store, with_from.store);
    }

    #[test]
    fn test_query() {
        let data = [69, 420, 123, 277, 1337, 1234, 5785, 156, 278];
        let mut seg_tree = LazySegTree::from_iter(
            data,
            0,
            0,
            |a, b| a + b,
            |val, lazy, l, r| val + lazy * (r - l + 1) as i32,
            |a, b| a + b,
        );

        for i in 0..data.len() {
            for j in i..data.len() {
                let sum: i32 = data[i..=j].iter().sum();
                assert_eq!(sum, seg_tree.query(i..=j));
            }
        }
    }

    #[test]
    fn test_update() {
        let mut data = [69, 420, 123, 277, 1337, 1234, 5785, 156, 278];
        let updates = [
            (2, 4, 7),
            (3, 5, 9),
            (4, 4, 2187),
            (1, 7, 123),
            (6, 8, 188),
            (0, 1, 17),
        ];

        let mut seg_tree = LazySegTree::from_iter(
            data,
            0,
            0,
            |a, b| a + b,
            |val, lazy, l, r| val + lazy * (r - l + 1) as i32,
            |a, b| a + b,
        );

        for (l, r, val) in updates {
            for i in l..=r {
                data[i] += val;
            }
            seg_tree.update(l..=r, val);

            for i in 0..data.len() {
                for j in i..data.len() {
                    let sum: i32 = data[i..=j].iter().sum();
                    assert_eq!(sum, seg_tree.query(i..=j));
                }
            }
        }
    }
}
