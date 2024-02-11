pub mod binary_indexed_tree;
pub mod disjoint_set;
pub mod doubling;
pub mod geometry;
pub mod input;
pub mod math;
pub mod modulus;
pub mod prime;
pub mod run_length_encoding;
pub mod structure;

///
/// 最大公約数
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

///
/// 最小公倍数
///
pub fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(18, 24), 6);
        assert_eq!(gcd(108, 56), 4);
        assert_eq!(gcd(8633, 6052), 89);
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(127, 45), 5715);
        assert_eq!(lcm(12, 18), 36);
    }
}
