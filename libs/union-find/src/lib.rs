#[derive(Debug)]
pub struct UnionFind {
    rank: Vec<usize>,
    parent: Vec<usize>,
    /// 重み
    diff_weight: Vec<isize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            rank: vec![0; n],
            parent: (0..n).collect::<Vec<_>>(),
            diff_weight: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            let r = self.find(self.parent[x]);

            self.diff_weight[x] += self.diff_weight[self.parent[x]];

            self.parent[x] = r;
        }

        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        self.union_weight(x, y, 0);
    }

    pub fn union_weight(&mut self, x: usize, y: usize, w: isize) {
        let mut w = w + self.weight(x) - self.weight(y);

        let mut x = self.find(self.parent[x]);
        let mut y = self.find(self.parent[y]);

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

    pub fn equiv(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn weight(&mut self, x: usize) -> isize {
        self.find(x);

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
    use super::UnionFind;

    #[test]
    fn test_disjoint_set() {
        let mut set = UnionFind::new(3);

        assert_eq!(set.equiv(0, 1), false);
        assert_ne!(set.find(0), set.find(1));

        set.union(0, 1);

        assert_eq!(set.equiv(0, 1), true);
        assert_eq!(set.find(0), set.find(1));

        set.union(0, 2);

        assert_eq!(set.equiv(0, 2), true);
        assert_eq!(set.find(0), set.find(1));
        assert_eq!(set.find(1), set.find(2));
    }
}
