//!
//! https://judge.yosupo.jp/problem/unionfind
//!

fn main() {
    let (n, q) = input_tuple_2();
    let t_u_v = input_tuple_3_vec(q);

    let mut set = DisjointSet::new_with(n as i64);

    for (t, u, v) in t_u_v {
        if t == 0 {
            set.unite(u, v);
        } else {
            println!("{}", if set.same(u, v) { 1 } else { 0 });
        }
    }
}

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
    use crate::{disjoint_set::DisjointSet, input::input_tuple_3_vec};

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

pub fn input_value<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n = buf.parse().unwrap();

    n
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

pub fn input_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let iter = buf.split_whitespace();

    let line = iter.map(|x| x.parse().unwrap()).collect();

    line
}

pub fn input_vec_2d<T>(n: usize) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // 二次元ベクタ

    let stdin = std::io::stdin();

    let mut a = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let iter = buf.split_whitespace();

        let line = iter.map(|x| x.parse().unwrap()).collect();

        a.push(line);
    }

    a
}

pub fn input_tuple_2_vec<T>(n: usize) -> Vec<(T, T)>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // タプルのベクタ

    let stdin = std::io::stdin();

    let mut s_t = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let s = iter.next().unwrap().parse().unwrap();
        let t = iter.next().unwrap().parse().unwrap();

        s_t.push((s, t));
    }

    s_t
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

pub fn input_char_vec() -> Vec<char> {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let x = buf.chars().collect();

    x
}
