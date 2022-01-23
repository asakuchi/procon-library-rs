//!
//! 素数
//!

///
/// n 以下の素数を返す
/// エラトステネスのふるい
///
pub fn get_prime(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    let mut list = Vec::with_capacity(n);

    for i in 2..=n {
        if is_prime[i] {
            list.push(i);

            for j in (i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    list
}
