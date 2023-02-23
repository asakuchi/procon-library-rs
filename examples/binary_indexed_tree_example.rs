//!
//! Binary Indexed Tree (BIT)
//!

use procon_library_rs::binary_indexed_tree::BinaryIndexedTree;
fn main() {
    // 転倒数を求める

    let n = 5;
    let a = vec![3, 1, 5, 4, 2];

    let mut bit = BinaryIndexedTree::new(n);

    let mut inversion = 0;

    for x in a {
        inversion += bit.sum(n) - bit.sum(x);

        bit.add(x, 1);
    }

    assert_eq!(inversion, 5);
}
