///
/// ダブリング
///
/// ABC167D
/// ```
/// use doubling::Doubling;
///
/// // 1回移動した先
/// let list = vec![5, 4, 1, 4, 2, 1];
///
/// let mut doubling = Doubling::new(727202214173249351, &list);
/// doubling.preprocess();
///
/// // 地点 0 から 727202214173249351 回移動した先は1
/// assert_eq!(doubling.get(727202214173249351, 0), 1);
/// ```
pub struct Doubling {
    n: usize,
    log_k: usize,
    /// doubling[k][i] : i番目から 2^k 進んだ町
    doubling: Vec<Vec<usize>>,
}

impl Doubling {
    ///
    /// 初期化
    ///
    /// * `k` - 最終的に求めたい step
    /// * `one_step` - 1 step でどこに移動するか
    ///
    pub fn new(k: usize, one_step: &Vec<usize>) -> Doubling {
        let n = one_step.len();
        let mut log_k = 0;

        while 1 << log_k <= k {
            log_k += 1;
        }

        let mut doubling = vec![vec![0; n]; log_k + 1];

        doubling[0] = one_step.clone();

        Doubling { n, log_k, doubling }
    }

    ///
    /// 事前処理
    ///
    pub fn preprocess(&mut self) {
        for j in 0..self.log_k {
            for i in 0..self.n {
                self.doubling[j + 1][i] = self.doubling[j][self.doubling[j][i]];
            }
        }
    }

    ///
    /// start から k step 目の場所を求める
    ///
    pub fn get(&self, k: usize, start: usize) -> usize {
        let mut current = start;

        for i in (0..self.log_k).rev() {
            if k & (1 << i) > 0 {
                current = self.doubling[i][current];
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn doubling_works() {
        let list = vec![2, 1, 3, 0];

        let mut doubling = Doubling::new(5, &list);
        doubling.preprocess();

        assert_eq!(doubling.get(5, 0), 3);

        let list = vec![5, 4, 1, 4, 2, 1];

        let mut doubling = Doubling::new(727202214173249351, &list);
        doubling.preprocess();

        assert_eq!(doubling.get(727202214173249351, 0), 1);
    }
}
