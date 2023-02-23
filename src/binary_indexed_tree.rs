//!
//! Binary Indexed Tree (BIT)
//!
//! 参考：<https://algo-logic.info/binary-indexed-tree/>
//!

///
/// Binary Indexed Tree (BIT)
///
pub struct BinaryIndexedTree {
    n: usize,
    bit: Vec<isize>,
}

impl BinaryIndexedTree {
    pub fn new(n: usize) -> BinaryIndexedTree {
        let bit = vec![0; n + 1];

        BinaryIndexedTree { n: n + 1, bit }
    }

    pub fn add(&mut self, i: usize, x: isize) {
        assert_ne!(i, 0, "i is 1-index");

        let mut index = i as isize;

        while (index as usize) < self.n {
            self.bit[index as usize] += x;

            // i & -i => i の最後の1のビット
            index += index & -index;
        }
    }

    pub fn sum(&self, i: usize) -> isize {
        assert_ne!(i, 0, "i is 1-index");

        let mut s = 0;

        let mut index = i as isize;

        while index > 0 {
            s += self.bit[index as usize];

            // i & -i => i の最後の1のビット
            index -= index & -index;
        }

        s
    }
}

#[cfg(test)]
mod test {
    use crate::binary_indexed_tree::BinaryIndexedTree;

    #[test]
    fn test_bit() {
        let mut tree = BinaryIndexedTree::new(5);

        tree.add(1, 10);
        tree.add(2, 20);
        tree.add(3, 30);
        tree.add(4, 40);
        tree.add(5, 50);

        assert_eq!(tree.sum(1), 10);
        assert_eq!(tree.sum(2), 30);
        assert_eq!(tree.sum(3), 60);
        assert_eq!(tree.sum(4), 100);
        assert_eq!(tree.sum(5), 150);
    }

    #[test]
    fn test_inversion() {
        assert_eq!(inversion(5, vec![3, 1, 5, 4, 2]), 5);
        assert_eq!(inversion(6, vec![1, 2, 3, 4, 5, 6]), 0);
        assert_eq!(inversion(7, vec![7, 6, 5, 4, 3, 2, 1]), 21);
        assert_eq!(
            inversion(
                20,
                vec![19, 11, 10, 7, 8, 9, 17, 18, 20, 4, 3, 15, 16, 1, 5, 14, 6, 2, 13, 12]
            ),
            114
        );
    }

    fn inversion(n: usize, a: Vec<usize>) -> isize {
        // 転倒数を求める

        let mut bit = BinaryIndexedTree::new(n);

        let mut inversion = 0;

        for x in a {
            inversion += bit.sum(n) - bit.sum(x);

            bit.add(x, 1);
        }

        inversion
    }
}
