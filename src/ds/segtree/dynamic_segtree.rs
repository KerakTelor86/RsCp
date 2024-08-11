type Ptr<T> = Option<Box<DynStNode<T>>>;

struct DynStNode<T: Clone> {
    left: Ptr<T>,
    right: Ptr<T>,
    value: T,
}

impl<T: Clone> DynStNode<T> {
    fn new(value: T) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }
}

pub struct DynamicSegTree<T: Clone, F>
where
    F: Fn(T, T) -> T,
{
    size: usize,
    nil: T,
    operation: F,
    root: Ptr<T>,
}

impl<T: Clone, F> DynamicSegTree<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(size: usize, nil: T, operation: F) -> Self {
        Self {
            size,
            nil: nil.clone(),
            operation,
            root: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn set(&mut self, pos: usize, value: T) {
        let node = self.root.take();
        self.root = self.set_impl(pos, value, node, 0, self.size - 1);
    }

    pub fn update(&mut self, pos: usize, value: T) {
        let node = self.root.take();
        self.root = self.update_impl(pos, value, node, 0, self.size - 1);
    }

    pub fn query(&self, left: usize, right: usize) -> T {
        self.query_impl(left, right, &self.root, 0, self.size - 1)
    }

    fn get_value(&self, ptr: &Ptr<T>) -> T {
        match ptr {
            Some(node) => node.value.clone(),
            None => self.nil.clone(),
        }
    }

    fn deref_or_default(&self, ptr: Ptr<T>) -> Box<DynStNode<T>> {
        match ptr {
            Some(x) => x,
            None => Box::new(DynStNode::new(self.nil.clone())),
        }
    }

    fn set_impl(
        &self,
        u: usize,
        w: T,
        cur: Ptr<T>,
        l: usize,
        r: usize,
    ) -> Ptr<T> {
        if u > r || u < l {
            return cur;
        }
        let mut cur = self.deref_or_default(cur);
        if u == l && u == r {
            cur.value = w;
            return Some(cur);
        }
        let m = (l + r) / 2;
        if u <= m {
            cur.left = self.set_impl(u, w, cur.left.take(), l, m);
        } else {
            cur.right = self.set_impl(u, w, cur.right.take(), m + 1, r);
        }
        cur.value = (self.operation)(
            self.get_value(&cur.left),
            self.get_value(&cur.right),
        );
        return Some(cur);
    }

    fn update_impl(
        &self,
        u: usize,
        w: T,
        cur: Ptr<T>,
        l: usize,
        r: usize,
    ) -> Ptr<T> {
        if u > r || u < l {
            return cur;
        }
        let mut cur = self.deref_or_default(cur);
        if u == l && u == r {
            cur.value = (self.operation)(cur.value, w);
            return Some(cur);
        }
        let m = (l + r) / 2;
        if u <= m {
            cur.left = self.update_impl(u, w, cur.left.take(), l, m);
        } else {
            cur.right = self.update_impl(u, w, cur.right.take(), m + 1, r);
        }
        cur.value = (self.operation)(
            self.get_value(&cur.left),
            self.get_value(&cur.right),
        );
        return Some(cur);
    }

    fn query_impl(
        &self,
        u: usize,
        v: usize,
        cur: &Ptr<T>,
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
        return (self.operation)(
            self.query_impl(u, v, &cur.left, l, m),
            self.query_impl(u, v, &cur.right, m + 1, r),
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query() {
        let data = [69, 420, 123, 277, 1337, 1234, 5785, 156, 278];

        let seg_tree = {
            let mut temp = DynamicSegTree::new(data.len(), 0, |a, b| a + b);
            for (i, val) in data.iter().enumerate() {
                temp.set(i, val.clone());
            }
            temp
        };

        for i in 0..data.len() {
            for j in i..data.len() {
                let sum: i32 = data[i..=j].iter().sum();
                assert_eq!(sum, seg_tree.query(i, j));
            }
        }
    }

    #[test]
    fn test_update() {
        let mut data = [0; 9];
        let updates = [(2, 7), (3, 9), (4, 2187), (7, 123), (2, 188), (0, 17)];

        let mut seg_tree = DynamicSegTree::new(data.len(), 0, |a, b| a + b);

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

    #[test]
    fn test_big_range() {
        const SIZE: usize = 1usize << 60;
        let mut seg_tree = DynamicSegTree::new(SIZE, 0, |a, b| a + b);
        assert_eq!(seg_tree.query(0, SIZE - 1), 0);

        seg_tree.set(0, 100);
        seg_tree.set(998244353, 69);
        seg_tree.set(SIZE - 3, 420);

        assert_eq!(seg_tree.query(0, 998244352), 100);
        assert_eq!(seg_tree.query(1, 998244353), 69);
        assert_eq!(seg_tree.query(0, 998244353), 169);
        assert_eq!(seg_tree.query(0, SIZE - 1), 589);
    }
}
