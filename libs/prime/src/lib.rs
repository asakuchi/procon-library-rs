//!
//! 素数
//!

///
/// n 以下の素数を返す
/// エラトステネスのふるい
///
/// O(n log log n)
///
/// ```
/// use prime::get_prime;
///
/// let primes = get_prime(10);
///
/// assert_eq!(primes, vec![2, 3, 5, 7,]);
/// ```
///
pub fn get_prime(n: usize) -> Vec<usize> {
    assert!(n >= 2, "n must be 2 or more");

    let mut is_prime = vec![true; n + 1];
    let mut list = Vec::with_capacity(n);

    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if is_prime[i] {
            list.push(i);

            for j in (i * 2..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    list
}

///
/// 素数判定
///
/// ```
/// use prime::is_prime;
///
/// assert!(is_prime(1_000_000_007))
/// ```
///
pub fn is_prime(x: usize) -> bool {
    if x == 2 {
        return true;
    }

    if x < 2 || x % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i = i + 2;
    }

    true
}

///
/// 素因数分解
///
/// ```
/// use prime::prime_factorize;
///
/// let primes = prime_factorize(24);
///
/// assert_eq!(primes, vec![2, 2, 2, 3]);
/// ```
pub fn prime_factorize(n: usize) -> Vec<usize> {
    let mut current = n;

    let mut list = Vec::new();

    {
        let mut i = 2;

        while i * i <= n {
            while current % i == 0 {
                list.push(i);
                current /= i;
            }

            if current == 1 {
                break;
            }

            i += 1;
        }

        if current != 1 {
            list.push(current);
        }
    }

    list
}

///
/// 高速な素因数分解（前処理）
///
/// see [`osa_k`]
///
pub fn pre_osa_k(n: usize) -> Vec<usize> {
    let mut min_factor: Vec<_> = (0..=n).collect();

    let mut i = 2;

    while i * i <= n {
        if min_factor[i] == i {
            for k in (i * 2..=n).step_by(i) {
                if min_factor[k] > i {
                    min_factor[k] = i;
                }
            }
        }

        i += 1;
    }

    min_factor
}

///
/// 高速な素因数分解
///
/// ```
/// use prime::{pre_osa_k, osa_k};
///
/// let min_factor = pre_osa_k(1_000_000);
/// let primes = osa_k(24, &min_factor);
///
/// assert_eq!(primes, vec![2, 2, 2, 3]);
/// ```
pub fn osa_k(m: usize, min_factor: &Vec<usize>) -> Vec<usize> {
    let mut k = m;

    let mut primes = Vec::new();

    while k >= 2 {
        primes.push(min_factor[k]);

        k /= min_factor[k];
    }

    primes
}

///
/// 約数列挙
///
/// N=10^6でK=240、N=10^9でK=1344、N=10^12でK=6720
///
/// ```
/// use prime::divisors;
///
/// let div = divisors(27);
///
/// assert_eq!(div, vec![1, 3, 9, 27]);
/// ```
pub fn divisors(n: usize) -> Vec<usize> {
    let mut list = Vec::new();

    {
        let mut i = 1;

        while i * i <= n {
            if n % i == 0 {
                list.push(i);

                if n / i != i {
                    list.push(n / i);
                }
            }

            i += 1;
        }
    }

    list.sort();

    list
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_prime() {
        let actual = get_prime(1_000);

        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997,
        ];

        assert_eq!(actual, expected, "素数の一覧が生成できていない");
    }

    #[test]
    fn test_is_prime() {
        // 素数
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(is_prime(997));
        assert!(is_prime(4759));
        assert!(is_prime(26399));
        assert!(is_prime(82339));
        assert!(is_prime(1_000_000_007));

        // 素数でない
        assert!(!is_prime(4));
        assert!(!is_prime(651));
        assert!(!is_prime(1437));
        assert!(!is_prime(79067));
        assert!(!is_prime(99997));
    }

    #[test]
    fn test_prime_factorize() {
        assert_eq!(prime_factorize(30248), vec![2, 2, 2, 19, 199]);
        assert_eq!(prime_factorize(64197), vec![3, 3, 7, 1019]);
        assert_eq!(prime_factorize(86767), vec![86767]);
        assert_eq!(prime_factorize(94624), vec![2, 2, 2, 2, 2, 2957]);
    }

    #[test]
    fn test_osa_k() {
        let min_factor = pre_osa_k(1_000_000);

        // let mut primes = osa_k(a[i], &min_factor);

        assert_eq!(osa_k(15866, &min_factor), vec![2, 7933]);
        assert_eq!(osa_k(20897, &min_factor), vec![20897]);
        assert_eq!(osa_k(45297, &min_factor), vec![3, 3, 7, 719]);
        assert_eq!(osa_k(70277, &min_factor), vec![31, 2267]);

        let n = 77526;
        assert_eq!(prime_factorize(n), osa_k(n, &min_factor));

        let n = 79039; // 素数
        assert_eq!(prime_factorize(n), osa_k(n, &min_factor));

        let n = 958550;
        assert_eq!(prime_factorize(n), osa_k(n, &min_factor));

        let n = 977821;
        assert_eq!(prime_factorize(n), osa_k(n, &min_factor));

        let n = 999978;
        assert_eq!(prime_factorize(n), osa_k(n, &min_factor));
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(10), vec![1, 2, 5, 10]);
        assert_eq!(divisors(13), vec![1, 13]);
        assert_eq!(divisors(16), vec![1, 2, 4, 8, 16]);
        assert_eq!(divisors(1_000_000_007), vec![1, 1_000_000_007]);
    }
}
