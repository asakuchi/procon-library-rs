// const MAX_SIZE: usize = 510000;

pub const PRIME_1_000_000_007: usize = 1_000_000_007;
pub const PRIME_998_244_353: usize = 998_244_353;

///
/// a^n (mod p) を求める
///
/// 繰り返し二乗法
///
pub fn mod_pow(a: usize, n: usize, p: usize) -> usize {
    if n == 0 {
        return 1 % p;
    }

    if n == 1 {
        return a % p;
    }

    if n % 2 == 1 {
        return (a * mod_pow(a, n - 1, p)) % p;
    }

    let t = mod_pow(a, n / 2, p);

    return (t * t) % p;
}

///
/// 1 / a (mod p) を求める
///
pub fn mod_inv(a: usize, p: usize) -> usize {
    return mod_pow(a, p - 2, p);
}

///
/// 順列 nPk (mod p) を求める
///
pub fn mod_perm(n: usize, k: usize, p: usize) -> usize {
    let mut ret = 1;

    for i in 0..k {
        ret = (ret * (n - i)) % p;
    }

    return ret;
}

///
/// 組み合わせ nCk (mod p) を求める
///
pub fn mod_comb(n: usize, k: usize, p: usize) -> usize {
    let a = mod_perm(n, k, p);
    let b = mod_perm(k, k, p);

    return (a * mod_inv(b, p)) % p;
}

/**
 * 重複組合せ
 *
 * https://www.geisya.or.jp/~mwm48961/kou2/s1combi5.htm
 *
 * https://atcoder.jp/contests/abc021/tasks/abc021_d
 */
pub fn mod_comb_with_repetition(n: usize, k: usize, p: usize) -> usize {
    mod_comb(n - 1 + k, n - 1, p)
}

/**
 * 重複順列
 *
 * https://www.geisya.or.jp/~mwm48961/kou2/rep_permu.htm
 */
pub fn mod_perm_with_repetition(n: usize, k: usize, p: usize) -> usize {
    mod_pow(n, k, p)
}

// ///
// /// 負の数にも対応した % 演算
// ///
// fn modulo(value: isize, m: isize) -> isize {
//     let mut result = value % m;

//     if result < 0 {
//         result += m;
//     }

//     result
// }

// // fn mod_inverse(a: isize, m: isize) -> isize {
// //     let extgcd = isize::extended_gcd(&a, &m);

// //     (m + extgcd.x % m) % m
// // }

// fn mod_inverse(a: isize, m: isize) -> isize {
//     let mut a = a;
//     let mut b = m;
//     let mut u = 1;
//     let mut v = 0;

//     while b > 0 {
//         let t = a / b;
//         a -= t * b;
//         std::mem::swap(&mut a, &mut b);
//         u -= t * v;
//         std::mem::swap(&mut u, &mut v);
//     }

//     u %= m;

//     if u < 0 {
//         u += m;
//     }

//     u
// }

// ///
// /// 組み合わせ 前処理
// /// n <= 10^7 まで
// ///
// fn combination_init(fac: &mut Vec<isize>, finv: &mut Vec<isize>, modulus: isize) {
//     fac[0] = 1;
//     fac[1] = 1;
//     finv[0] = 1;
//     finv[1] = 1;

//     let mut inv = vec![0; MAX_SIZE];
//     inv[1] = 1;

//     for i in 2..MAX_SIZE {
//         fac[i] = fac[i - 1] * i as isize % modulus;
//         inv[i] = modulus - inv[modulus as usize % i] * (modulus / i as isize) % modulus;
//         finv[i] = finv[i - 1] * inv[i] % modulus;
//     }
// }

// ///
// /// 組み合わせ
// /// n <= 10^7 まで
// ///
// /// ```
// /// use procon
// ///
// /// let mut fac = vec![0; MAX_SIZE];
// /// let mut finv = vec![0; MAX_SIZE];
// ///
// /// let n = 4;
// /// let k = 2;
// ///
// /// // nCk
// /// combination_init(&mut fac, &mut finv);
// /// println!("{}", combination(n, k, &fac, &finv));
// /// ```
// fn combination(n: isize, k: isize, fac: &Vec<isize>, finv: &Vec<isize>, modulus: isize) -> isize {
//     if n < k {
//         return 0;
//     }
//     if n < 0 || k < 0 {
//         return 0;
//     }

//     fac[n as usize] * (finv[k as usize] * finv[(n - k) as usize] % modulus) % modulus
// }
