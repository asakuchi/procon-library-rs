#[derive(Debug)]
pub struct DisjointSet {
    rank: Vec<usize>,
    parent: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> DisjointSet {
        DisjointSet {
            rank: vec![0; n],
            parent: (0..n).collect::<Vec<_>>(),
        }
    }

    pub fn find_set(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find_set(self.parent[x]);
        }

        self.parent[x]
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let a = self.find_set(self.parent[x]);
        let b = self.find_set(self.parent[y]);

        if self.rank[a] > self.rank[b] {
            self.parent[b] = a;
        } else {
            self.parent[a] = b;
            if self.rank[a] == self.rank[b] {
                self.rank[b] += 1;
            }
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find_set(x) == self.find_set(y)
    }
}

#[cfg(test)]
mod tests {
    use crate::disjoint_set::DisjointSet;

    #[test]
    fn test_disjoint_set() {
        let mut set = DisjointSet::new(3);

        assert_eq!(set.same(0, 1), false);
        assert_ne!(set.find_set(0), set.find_set(1));

        set.unite(0, 1);

        assert_eq!(set.same(0, 1), true);
        assert_eq!(set.find_set(0), set.find_set(1));

        set.unite(0, 2);

        assert_eq!(set.same(0, 2), true);
        assert_eq!(set.find_set(0), set.find_set(1));
        assert_eq!(set.find_set(1), set.find_set(2));
    }
}
