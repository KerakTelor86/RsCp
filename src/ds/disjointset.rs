pub struct DisjointSet {
    parent: Vec<usize>,
    count: Vec<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            count: vec![1; size],
        }
    }

    pub fn get_root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.get_root(self.parent[x]);
            self.parent[x]
        }
    }

    pub fn make_root(&mut self, x: usize) {
        let root = self.get_root(x);
        if root != x {
            self.parent[x] = x;
            self.count[x] = self.count[root];
            self.parent[root] = x;
        }
    }

    pub fn join(&mut self, x: usize, y: usize) {
        let x = self.get_root(x);
        let y = self.get_root(y);
        if x == y {
            return;
        }
        let (x, y) = if self.count[x] < self.count[y] {
            (x, y)
        } else {
            (y, x)
        };
        self.count[y] += self.count[x];
        self.parent[x] = y;
    }

    pub fn get_count(&mut self, x: usize) -> usize {
        let x = self.get_root(x);
        self.count[x]
    }

    pub fn len(&self) -> usize {
        self.count.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dsu() {
        let mut dsu = DisjointSet::new(10);
        assert_eq!(dsu.len(), 10);

        for i in 0..10 {
            assert_eq!(dsu.get_root(i), i);
        }

        let edges = [
            (0, 1),
            (1, 2),
            (0, 2),
            (3, 4),
            (5, 3),
            (5, 6),
            (4, 6),
            (9, 8),
            (7, 8),
        ];

        for (u, v) in edges {
            dsu.join(u, v);
        }

        let expected_root = [0, 0, 0, 3, 3, 3, 3, 9, 9, 9];
        let expected_count = [3, 3, 3, 4, 4, 4, 4, 3, 3, 3];
        for i in 0..10 {
            assert_eq!(dsu.get_root(i), expected_root[i]);
            assert_eq!(dsu.get_count(i), expected_count[i]);
        }

        for i in 0..10 {
            dsu.make_root(i);
            assert_eq!(dsu.get_root(i), i);
            assert_eq!(dsu.get_count(i), expected_count[i]);
        }
    }
}
