use crate::misc::range::RangeWrapper;
use std::rc::Rc;

type PstPtr<T> = Option<Rc<PtrStNode<T>>>;

#[derive(Clone)]
struct PtrStNode<T: Clone> {
    left: PstPtr<T>,
    right: PstPtr<T>,
    value: T,
}

impl<T: Clone> PtrStNode<T> {
    fn new(value: T) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }
}

#[derive(Clone)]
pub struct PersistentSegTree<T: Clone, F>
where
    F: Fn(T, T) -> T,
{
    size: usize,
    nil: T,
    operation: Rc<F>,
    root: PstPtr<T>,
}

impl<T: Clone, F> PersistentSegTree<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(size: usize, nil: T, operation: F) -> Self {
        Self {
            size,
            nil,
            operation: Rc::new(operation),
            root: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn set(&self, pos: usize, value: T) -> Self {
        let root =
            self.set_impl(pos, value, self.root.clone(), 0, self.size - 1);
        Self {
            size: self.size.clone(),
            nil: self.nil.clone(),
            operation: self.operation.clone(),
            root,
        }
    }

    pub fn update(&self, pos: usize, value: T) -> Self {
        let root =
            self.update_impl(pos, value, self.root.clone(), 0, self.size - 1);
        Self {
            size: self.size.clone(),
            nil: self.nil.clone(),
            operation: self.operation.clone(),
            root,
        }
    }

    pub fn query(&self, range: impl RangeWrapper<usize>) -> T {
        let (left, right) = range.closed_bounds();
        self.query_impl(left, right, &self.root, 0, self.size - 1)
    }

    fn get_value(&self, ptr: &PstPtr<T>) -> T {
        match ptr {
            Some(node) => node.value.clone(),
            None => self.nil.clone(),
        }
    }

    fn deref_or_default(&self, ptr: PstPtr<T>) -> Rc<PtrStNode<T>> {
        match ptr {
            Some(x) => x,
            None => Rc::new(PtrStNode::new(self.nil.clone())),
        }
    }

    fn set_impl(
        &self,
        u: usize,
        w: T,
        cur: PstPtr<T>,
        l: usize,
        r: usize,
    ) -> PstPtr<T> {
        if u > r || u < l {
            return cur;
        }
        let mut cur = self.deref_or_default(cur);
        let data = Rc::make_mut(&mut cur);
        if u == l && u == r {
            data.value = w;
            return Some(cur);
        }
        let m = (l + r) / 2;
        if u <= m {
            data.left = self.set_impl(u, w, data.left.take(), l, m);
        } else {
            data.right = self.set_impl(u, w, data.right.take(), m + 1, r);
        }
        data.value = (self.operation)(
            self.get_value(&data.left),
            self.get_value(&data.right),
        );
        Some(cur)
    }

    fn update_impl(
        &self,
        u: usize,
        w: T,
        cur: PstPtr<T>,
        l: usize,
        r: usize,
    ) -> PstPtr<T> {
        if u > r || u < l {
            return cur;
        }
        let mut cur = self.deref_or_default(cur);
        let data = Rc::make_mut(&mut cur);
        if u == l && u == r {
            data.value = (self.operation)(data.value.clone(), w);
            return Some(cur);
        }
        let m = (l + r) / 2;
        if u <= m {
            data.left = self.update_impl(u, w, data.left.take(), l, m);
        } else {
            data.right = self.update_impl(u, w, data.right.take(), m + 1, r);
        }
        data.value = (self.operation)(
            self.get_value(&data.left),
            self.get_value(&data.right),
        );
        Some(cur)
    }

    fn query_impl(
        &self,
        u: usize,
        v: usize,
        cur: &PstPtr<T>,
        l: usize,
        r: usize,
    ) -> T {
        if u > r || v < l {
            return self.nil.clone();
        }
        let Some(cur) = cur else {
            return self.nil.clone();
        };
        if u <= l && v >= r {
            return cur.value.clone();
        }
        let m = (l + r) / 2;
        (self.operation)(
            self.query_impl(u, v, &cur.left, l, m),
            self.query_impl(u, v, &cur.right, m + 1, r),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query() {
        let data = [69, 420, 123, 277, 1337, 1234, 5785, 156, 278];

        let seg_tree = {
            let mut temp = PersistentSegTree::new(data.len(), 0, |a, b| a + b);
            for (i, val) in data.iter().enumerate() {
                temp = temp.set(i, val.clone());
            }
            temp
        };

        for i in 0..data.len() {
            for j in i..data.len() {
                let sum: i32 = data[i..=j].iter().sum();
                assert_eq!(sum, seg_tree.query(i..=j));
            }
        }
    }

    #[test]
    fn test_update() {
        let mut data = [0; 9];
        let updates = [(2, 7), (3, 9), (4, 2187), (7, 123), (2, 188), (0, 17)];

        let mut seg_tree = PersistentSegTree::new(data.len(), 0, |a, b| a + b);

        for (idx, val) in updates {
            data[idx] += val;
            seg_tree = seg_tree.update(idx, val);

            for i in 0..data.len() {
                for j in i..data.len() {
                    let sum: i32 = data[i..=j].iter().sum();
                    assert_eq!(sum, seg_tree.query(i..=j));
                }
            }
        }
    }

    #[test]
    fn test_big_range() {
        const SIZE: usize = 1usize << 60;
        let mut seg_tree = PersistentSegTree::new(SIZE, 0, |a, b| a + b);
        assert_eq!(seg_tree.query(0..SIZE), 0);

        seg_tree = seg_tree.set(0, 100);
        seg_tree = seg_tree.set(998244353, 69);
        seg_tree = seg_tree.set(SIZE - 3, 420);

        assert_eq!(seg_tree.query(0..=998244352), 100);
        assert_eq!(seg_tree.query(1..=998244353), 69);
        assert_eq!(seg_tree.query(0..=998244353), 169);
        assert_eq!(seg_tree.query(0..SIZE), 589);
    }

    #[test]
    fn test_persistence() {
        let updates = [(2, 7), (3, 9), (4, 2187), (7, 123), (2, 188), (0, 17)];

        let mut data = [0; 9];
        let mut seg_tree = PersistentSegTree::new(data.len(), 0, |a, b| a + b);

        let mut data_history = Vec::new();
        let mut seg_tree_history = Vec::new();

        for (idx, val) in updates {
            data[idx] += val;
            data_history.push(data);

            seg_tree = seg_tree.update(idx, val);
            seg_tree_history.push(seg_tree.clone());
        }

        for (data, seg_tree) in
            data_history.into_iter().zip(seg_tree_history.into_iter())
        {
            for i in 0..seg_tree.len() {
                for j in i..seg_tree.len() {
                    let sum: i32 = data[i..=j].iter().sum();
                    assert_eq!(sum, seg_tree.query(i..=j));
                }
            }
        }
    }
}
