#[derive(Debug)]
pub struct DisjointSet {
    rank: Vec<i64>,
    p: Vec<i64>,
}

impl DisjointSet {
    pub fn new() -> DisjointSet {
        DisjointSet {
            rank: vec![-1; 1_000_000],
            p: vec![-1; 1_000_000],
        }
    }

    pub fn new_with(size: i64) -> DisjointSet {
        let mut set = DisjointSet {
            rank: vec![-1; 1_000_000],
            p: vec![-1; 1_000_000],
        };

        for i in 0..size {
            set.make_set(i);
        }

        set
    }

    pub fn make_set(&mut self, x: i64) {
        self.p[x as usize] = x;
        self.rank[x as usize] = 0;
    }

    pub fn find_set(&mut self, x: i64) -> i64 {
        if x != self.p[x as usize] {
            self.p[x as usize] = self.find_set(self.p[x as usize]);
        }

        self.p[x as usize]
    }

    pub fn unite(&mut self, x: i64, y: i64) {
        let a = self.find_set(self.p[x as usize]);
        let b = self.find_set(self.p[y as usize]);

        if self.rank[a as usize] > self.rank[b as usize] {
            self.p[b as usize] = a;
        } else {
            self.p[a as usize] = b;
            if self.rank[a as usize] == self.rank[b as usize] {
                self.rank[b as usize] += 1;
            }
        }
    }

    pub fn same(&mut self, x: i64, y: i64) -> bool {
        self.find_set(x) == self.find_set(y)
    }
}

#[cfg(test)]
mod tests {
    use crate::disjoint_set::DisjointSet;

    #[test]
    fn test_disjoint_set() {
        let mut set = DisjointSet::new();

        set.make_set(1);
        set.make_set(2);

        assert_eq!(set.same(1, 2), false);
        assert_ne!(set.find_set(1), set.find_set(2));

        set.unite(1, 2);

        assert_eq!(set.same(1, 2), true);
        assert_eq!(set.find_set(1), set.find_set(2));

        set.make_set(3);
        set.unite(1, 3);

        assert_eq!(set.same(1, 3), true);
        assert_eq!(set.find_set(1), set.find_set(2));
        assert_eq!(set.find_set(2), set.find_set(3));
    }
}
