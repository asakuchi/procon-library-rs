//
// TODO: 剰余や半環へ対応したい。
//
mod matrix {
    ///
    /// 行列
    ///
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Matrix {
        pub data: Vec<Vec<usize>>,
        pub row: usize,
        pub column: usize,
    }

    impl Matrix {
        pub fn new(data: Vec<Vec<usize>>) -> Self {
            let row = data.len();
            let column = data[0].len();

            Self { data, row, column }
        }

        pub fn dot(&self, rhs: &Matrix) -> Self {
            if self.column != rhs.row {
                panic!("self.column != rhs.row");
            }

            let row = self.row;
            let column = rhs.column;

            let mut data = vec![vec![0; column]; row];

            for i in 0..row {
                for j in 0..column {
                    for k in 0..self.column {
                        data[i][j] += self.data[i][k] * rhs.data[k][j];
                        // data[i][j] %= MOD;
                    }
                }
            }

            Self { data, row, column }
        }

        ///
        /// 行列累乗
        ///
        pub fn mat_pow(&self, x: usize) -> Self {
            if x == 0 {
                // 単位行列
                return Self::identity(self.column);
            }

            if x == 1 {
                return self.clone();
            }

            let mut t = self.mat_pow(x / 2);

            t = t.dot(&t);
            // t %= MOD;

            if x % 2 == 1 {
                t = t.dot(&self);
                // t %= MOD;
            }

            // 実数を扱う場合は誤差に注意
            // // 正規化
            // let p = t[(0, 0)];
            // let p_not = t[(0, 1)];

            // t *= 1. / (p + p_not);

            t
        }

        ///
        /// 単位行列
        ///
        pub fn identity(size: usize) -> Self {
            let mut data = Vec::new();

            for i in 0..size {
                let line = (0..size)
                    .map(|j| if i == j { 1 } else { 0 })
                    .collect::<Vec<_>>();
                data.push(line);
            }

            Self {
                data,
                row: size,
                column: size,
            }
        }
    }

    impl std::ops::Rem<usize> for Matrix {
        type Output = Self;

        fn rem(self, modulus: usize) -> Self::Output {
            let mut matrix = self.clone();

            matrix %= modulus;

            matrix
        }
    }

    impl std::ops::RemAssign<usize> for Matrix {
        fn rem_assign(&mut self, modulus: usize) {
            for i in 0..self.row {
                for j in 0..self.column {
                    self.data[i][j] %= modulus;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn test_matrix() {
        let matrix_1 = Matrix::new(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
        let matrix_2 = Matrix::new(vec![vec![7, 8], vec![9, 10]]);

        assert_eq!(
            matrix_1.dot(&matrix_2).data,
            vec![vec![25, 28], vec![57, 64], vec![89, 100]]
        );
    }
}
