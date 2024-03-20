use std::io;
use std::str;

pub struct Input<R> {
    reader: R,
    buffer: Vec<String>,
}

impl Input<std::io::StdinLock<'static>> {
    pub fn stdio() -> Self {
        let stdin = std::io::stdin();
        let scan = Input::new(stdin.lock());

        scan
    }
}

impl<R: io::BufRead> Input<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    pub fn read<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    pub fn usize1(&mut self) -> usize {
        self.read::<usize>() - 1
    }

    pub fn isize1(&mut self) -> isize {
        self.read::<isize>() - 1
    }

    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }

    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io() {
        let input = br"10
        1 2 3 4 5 6 7 8 9 10
        10
        20
        30
        40
        50
        abcde fghij
        
        3.14 -1592
        no_empty_line

        5 4
        ";
        let mut sc = Input::new(&input[..]);

        let n: usize = sc.read();
        assert_eq!(n, 10);

        let a: Vec<u64> = sc.vec(n);
        assert_eq!(&a, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let b: Vec<usize> = sc.vec(5);
        assert_eq!(&b, &[10, 20, 30, 40, 50]);

        let s = sc.chars();
        let t = sc.chars();
        assert_eq!(&s, &['a', 'b', 'c', 'd', 'e']);
        assert_eq!(&t, &['f', 'g', 'h', 'i', 'j']);

        let f: f64 = sc.read();
        assert_eq!(f, 3.14);

        let neg: i64 = sc.read();
        assert_eq!(neg, -1592);

        let s = sc.read::<String>();
        assert_eq!(&s, "no_empty_line");

        let x = sc.usize1();
        assert_eq!(x, 4);

        let y = sc.isize1();
        assert_eq!(y, 3);
    }
}
