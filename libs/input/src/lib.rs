use std::io;
use std::str;

pub trait ParseLine {
    fn parse_line<R: io::BufRead>(s: &mut Input<R>) -> Self;
}

macro_rules! impl_parse_line {
    ($($t:ty),*) => {
        $(impl ParseLine for $t {
            fn parse_line<R: io::BufRead>(s: &mut Input<R>) -> Self {
                s.token()
            }
        })*
    };
}
impl_parse_line!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, String, char);

macro_rules! impl_parse_line_tuple {
    ($($t:ident),*) => {
        impl<$($t: ParseLine),*> ParseLine for ($($t,)*) {
            fn parse_line<R: io::BufRead>(s: &mut Input<R>) -> Self {
                ($($t::parse_line(s),)*)
            }
        }
    };
}

impl_parse_line_tuple!(T0, T1);
impl_parse_line_tuple!(T0, T1, T2);
impl_parse_line_tuple!(T0, T1, T2, T3);
impl_parse_line_tuple!(T0, T1, T2, T3, T4);
impl_parse_line_tuple!(T0, T1, T2, T3, T4, T5);
impl_parse_line_tuple!(T0, T1, T2, T3, T4, T5, T6);
impl_parse_line_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
impl_parse_line_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_parse_line_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);

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

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    pub fn read<T: ParseLine>(&mut self) -> T {
        ParseLine::parse_line(self)
    }

    pub fn usize1(&mut self) -> usize {
        self.token::<usize>() - 1
    }

    pub fn isize1(&mut self) -> isize {
        self.token::<isize>() - 1
    }

    pub fn chars(&mut self) -> Vec<char> {
        self.token::<String>().chars().collect()
    }

    pub fn vec<T: ParseLine>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| ParseLine::parse_line(self)).collect()
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
        let mut input = Input::new(&input[..]);

        let n: usize = input.read();
        assert_eq!(n, 10);

        let a: Vec<u64> = input.vec(n);
        assert_eq!(&a, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let b: Vec<usize> = input.vec(5);
        assert_eq!(&b, &[10, 20, 30, 40, 50]);

        let s = input.chars();
        let t = input.chars();
        assert_eq!(&s, &['a', 'b', 'c', 'd', 'e']);
        assert_eq!(&t, &['f', 'g', 'h', 'i', 'j']);

        let f: f64 = input.read();
        assert_eq!(f, 3.14);

        let neg: i64 = input.read();
        assert_eq!(neg, -1592);

        let s = input.read::<String>();
        assert_eq!(&s, "no_empty_line");

        let x = input.usize1();
        assert_eq!(x, 4);

        let y = input.isize1();
        assert_eq!(y, 3);
    }
}
