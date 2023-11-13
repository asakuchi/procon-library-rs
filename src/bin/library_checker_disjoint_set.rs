//!
//! https://judge.yosupo.jp/problem/unionfind
//!

use procon_library_rs::disjoint_set::DisjointSet;

fn main() {
    let (n, q) = input_tuple_2();
    let t_u_v = input_tuple_3_vec(q);

    let mut set = DisjointSet::new(n);

    for (t, u, v) in t_u_v {
        if t == 0 {
            set.unite(u, v);
        } else {
            println!("{}", if set.same(u, v) { 1 } else { 0 });
        }
    }
}

pub fn input_tuple_2<T>() -> (T, T)
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();

    (n, m)
}

pub fn input_tuple_3_vec<T>(n: usize) -> Vec<(T, T, T)>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // タプルのベクタ

    let stdin = std::io::stdin();

    let mut s_t_d = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let s = iter.next().unwrap().parse().unwrap();
        let t = iter.next().unwrap().parse().unwrap();
        let d = iter.next().unwrap().parse().unwrap();

        s_t_d.push((s, t, d));
    }

    s_t_d
}
