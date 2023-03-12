pub mod binary_indexed_tree;
pub mod disjoint_set;
pub mod doubling;
pub mod geometry;
pub mod input;
pub mod math;
pub mod modulus;
pub mod prime;
pub mod run_length_encoding;

///
/// ユークリッドの互除法
///
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    }
}
