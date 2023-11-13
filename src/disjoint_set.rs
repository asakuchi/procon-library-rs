#[derive(Debug)]
pub struct DisjointSet {
    rank: Vec<usize>,
    parent: Vec<usize>,
    /// 重み
    diff_weight: Vec<isize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> DisjointSet {
        DisjointSet {
            rank: vec![0; n],
            parent: (0..n).collect::<Vec<_>>(),
            diff_weight: vec![0; n],
        }
    }

    pub fn find_set(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            let r = self.find_set(self.parent[x]);

            self.diff_weight[x] += self.diff_weight[self.parent[x]];

            self.parent[x] = r;
        }

        self.parent[x]
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        self.unite_weight(x, y, 0);
    }

    pub fn unite_weight(&mut self, x: usize, y: usize, w: isize) {
        let mut w = w + self.weight(x) - self.weight(y);

        let mut x = self.find_set(self.parent[x]);
        let mut y = self.find_set(self.parent[y]);

        // rank[a] > rank[b] となるように swap
        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
            w = -w;
        }

        if self.rank[x] == self.rank[y] {
            self.rank[x] += 1;
        }

        self.parent[y] = x;

        self.diff_weight[y] = w;
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find_set(x) == self.find_set(y)
    }

    pub fn weight(&mut self, x: usize) -> isize {
        self.find_set(x);

        self.diff_weight[x]
    }

    ///
    /// ノード間の距離を返す
    ///
    /// （重みつき UnionFind）
    ///
    pub fn diff(&mut self, x: usize, y: usize) -> isize {
        self.weight(y) - self.weight(x)
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
