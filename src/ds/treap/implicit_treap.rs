use crate::rand::rand::Rand;
use crate::rand::rng::wyrand::WyRand;
use crate::rand::traits::RandNext;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;
use std::vec::IntoIter;

type TreapPtr<T> = Option<Box<TreapNode<T>>>;

#[derive(Clone)]
struct TreapNode<T: Clone> {
    val: T,
    cumulative_val: T,
    left: TreapPtr<T>,
    right: TreapPtr<T>,
    priority: u32,
    size: usize,
    flip: bool,
}

impl<T: Clone> TreapNode<T> {
    fn new<R: RandNext<u32>>(val: T, rand: &Rc<RefCell<R>>) -> Self {
        Self {
            val: val.clone(),
            cumulative_val: val,
            left: None,
            right: None,
            priority: rand.borrow_mut().next(),
            size: 1,
            flip: false,
        }
    }
}

#[derive(Clone)]
pub struct ImplicitTreap<T: Clone, F: Fn(T, T) -> T, R: RandNext<u32>> {
    root: TreapPtr<T>,
    rand: Rc<RefCell<R>>,
    operation: Rc<F>,
}

impl<T: Clone, F: Fn(T, T) -> T> ImplicitTreap<T, F, Rand<WyRand, 8>> {
    pub fn new(operation: F) -> Self {
        Self::with_rand(Rand::default(), operation)
    }

    pub fn from_iter<U: IntoIterator<Item = T>>(iter: U, operation: F) -> Self {
        let mut treap = Self::with_rand(Rand::default(), operation);
        for val in iter {
            treap.push(val);
        }
        treap
    }
}

impl<T: Clone, F: Fn(T, T) -> T, R: RandNext<u32>> ImplicitTreap<T, F, R> {
    pub fn with_rand(rand: R, operation: F) -> Self {
        Self {
            root: None,
            rand: Rc::new(RefCell::new(rand)),
            operation: Rc::new(operation),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            let len = self.len() - 1;
            let root = self.root.take();
            let (root, popped) = self.node_split(root, len);
            self.root = root;
            Some(popped?.val)
        }
    }

    pub fn set(&mut self, index: usize, val: T) {
        assert!(index < self.len());

        let root = self.root.take();
        let (left, right) = self.node_split(root, index);
        let (target, right) = self.node_split(right, 1);

        let mut target = target.unwrap();
        target.val = val;
        self.node_update(&mut target);

        self.root = self.node_merge(self.node_merge(left, Some(target)), right);
    }

    pub fn query(&mut self, range: Range<usize>) -> T {
        assert!(range.end <= self.len());
        let root = self.root.take();

        let (left, right) = self.node_split(root, range.start);
        let (target, right) = self.node_split(right, range.end - range.start);

        let target = target.unwrap();
        let ret = target.cumulative_val.clone();

        self.root = self.node_merge(self.node_merge(left, Some(target)), right);
        ret
    }

    pub fn update(&mut self, index: usize, val: T) {
        assert!(index < self.len());
        let root = self.root.take();

        let (left, right) = self.node_split(root, index);
        let (target, right) = self.node_split(right, 1);

        let mut target = target.unwrap();
        target.val = (self.operation)(target.val.clone(), val);
        self.node_update(&mut target);

        self.root = self.node_merge(self.node_merge(left, Some(target)), right);
    }

    pub fn len(&self) -> usize {
        Self::node_len(&self.root)
    }

    pub fn push(&mut self, val: T) {
        self.insert(self.len(), val);
    }

    pub fn insert(&mut self, index: usize, val: T) {
        assert!(index <= self.len());
        let root = self.root.take();

        let (left, right) = self.node_split(root, index);
        let target = Box::new(TreapNode::new(val, &self.rand));
        self.root = self.node_merge(self.node_merge(left, Some(target)), right);
    }

    pub fn remove(&mut self, range: Range<usize>) {
        assert!(range.end <= self.len());
        let root = self.root.take();

        let (left, right) = self.node_split(root, range.start);
        let (target, right) = self.node_split(right, range.end - range.start);

        drop(target);

        self.root = self.node_merge(left, right);
    }

    pub fn reverse(&mut self, range: Range<usize>) {
        assert!(range.end <= self.len());
        let root = self.root.take();

        let (left, right) = self.node_split(root, range.start);
        let (mut target, right) =
            self.node_split(right, range.end - range.start);

        if let Some(target) = &mut target {
            target.flip = !target.flip;
        }

        self.root = self.node_merge(self.node_merge(left, target), right);
    }

    pub fn append(&mut self, other: Self) {
        let root = self.root.take();
        self.root = self.node_merge(root, other.root);
    }

    pub fn split(mut self, left_size: usize) -> (Self, Self) {
        assert!(left_size <= self.len());
        let root = self.root.take();
        let (left, right) = self.node_split(root, left_size);
        self.root = left;
        let other = Self {
            root: right,
            operation: self.operation.clone(),
            rand: self.rand.clone(),
        };
        (self, other)
    }

    fn node_len(node: &TreapPtr<T>) -> usize {
        match node {
            None => 0,
            Some(node) => node.size,
        }
    }

    fn node_propagate(node: &mut TreapNode<T>) {
        if node.flip {
            let temp = node.left.take();
            node.left = node.right.take();
            node.right = temp;
            for child in [&mut node.left, &mut node.right] {
                if let Some(child) = child {
                    child.flip = !child.flip;
                }
            }
            node.flip = false;
        }
    }

    fn node_update(&self, node: &mut TreapNode<T>) {
        node.cumulative_val = node.val.clone();
        node.size = 1;
        for child in [&mut node.left, &mut node.right] {
            if let Some(child) = child {
                Self::node_propagate(child);
                node.cumulative_val = (self.operation)(
                    node.cumulative_val.clone(),
                    child.cumulative_val.clone(),
                );
                node.size += child.size;
            }
        }
    }

    fn node_split(
        &self,
        node: TreapPtr<T>,
        size: usize,
    ) -> (TreapPtr<T>, TreapPtr<T>) {
        assert!(size <= Self::node_len(&node));
        match node {
            None => (None, None),
            Some(mut node) => {
                Self::node_propagate(&mut node);
                let left_size = Self::node_len(&node.left);
                if size <= left_size {
                    let (left, mid) = self.node_split(node.left, size);
                    node.left = mid;
                    self.node_update(&mut node);
                    (left, Some(node))
                } else {
                    let (mid, right) =
                        self.node_split(node.right, size - left_size - 1);
                    node.right = mid;
                    self.node_update(&mut node);
                    (Some(node), right)
                }
            }
        }
    }

    fn node_merge(&self, left: TreapPtr<T>, right: TreapPtr<T>) -> TreapPtr<T> {
        let Some(mut left) = left else {
            return right;
        };
        Self::node_propagate(&mut left);
        let Some(mut right) = right else {
            return Some(left);
        };
        Self::node_propagate(&mut right);
        if left.priority <= right.priority {
            left.right = self.node_merge(left.right, Some(right));
            self.node_update(&mut left);
            Some(left)
        } else {
            right.left = self.node_merge(Some(left), right.left);
            self.node_update(&mut right);
            Some(right)
        }
    }

    fn collect_inorder(node: TreapPtr<T>, out: &mut Vec<T>) {
        let Some(mut node) = node else {
            return;
        };
        Self::node_propagate(&mut node);
        Self::collect_inorder(node.left, out);
        out.push(node.val);
        Self::collect_inorder(node.right, out);
    }
}

impl<T: Clone, F: Fn(T, T) -> T, R: RandNext<u32>> IntoIterator
    for ImplicitTreap<T, F, R>
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        let mut vec = Vec::with_capacity(self.len());
        Self::collect_inorder(self.root.take(), &mut vec);
        vec.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push() {
        let mut treap = ImplicitTreap::new(|a: i32, b: i32| a + b);
        treap.push(1);
        treap.push(2);
        treap.push(3);
        treap.push(4);
        treap.push(5);

        assert_eq!(treap.into_iter().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_pop() {
        let mut treap = ImplicitTreap::new(|a: i32, b: i32| a + b);
        assert_eq!(treap.pop(), None);

        treap.push(1);
        treap.push(2);
        treap.push(3);
        treap.push(4);
        treap.push(5);

        assert_eq!(treap.pop(), Some(5));
        assert_eq!(treap.pop(), Some(4));
        assert_eq!(treap.pop(), Some(3));
        assert_eq!(treap.pop(), Some(2));
        assert_eq!(treap.pop(), Some(1));
        assert_eq!(treap.pop(), None);
    }

    #[test]
    fn test_split_append_reverse() {
        let mut treap = ImplicitTreap::from_iter(0..9, |a: i32, b: i32| a + b);
        treap.reverse(1..8);
        let (left, right) = treap.split(3);
        let (mid, mut right) = right.split(3);
        right.append(mid);
        right.append(left);

        let vec = right.into_iter().collect::<Vec<_>>();
        assert_eq!(vec, vec![2, 1, 8, 5, 4, 3, 0, 7, 6]);
    }

    #[test]
    fn test_remove() {
        let mut treap = ImplicitTreap::from_iter(0..10, |a: i32, b: i32| a + b);

        treap.remove(6..9);
        treap.remove(2..4);

        let vec = treap.into_iter().collect::<Vec<_>>();
        assert_eq!(vec, vec![0, 1, 4, 5, 9])
    }

    #[test]
    fn test_set() {
        let mut treap = ImplicitTreap::from_iter([0; 10], |a, b| a + b);
        for i in 0..10 {
            treap.set(i, 69);
        }
        assert_eq!(treap.into_iter().collect::<Vec<_>>(), vec![69; 10]);
    }

    #[test]
    fn test_query() {
        let data = [69, 420, 123, 277, 1337, 1234, 5785, 156, 278];
        let mut treap = ImplicitTreap::from_iter(data, |a, b| a + b);

        for i in 0..data.len() {
            for j in i + 1..=data.len() {
                let sum: i32 = data[i..j].iter().sum();
                assert_eq!(sum, treap.query(i..j));
            }
        }
    }

    #[test]
    fn test_update() {
        let mut data = [69, 420, 123, 277, 1337, 1234, 5785, 156, 278];
        let updates = [(2, 7), (3, 9), (4, 2187), (7, 123), (2, 188), (0, 17)];

        let mut treap = ImplicitTreap::from_iter(data, |a, b| a + b);

        for (idx, val) in updates {
            data[idx] += val;
            treap.update(idx, val);

            for i in 0..data.len() {
                for j in i + 1..=data.len() {
                    let sum: i32 = data[i..j].iter().sum();
                    assert_eq!(sum, treap.query(i..j));
                }
            }
        }
    }
}
