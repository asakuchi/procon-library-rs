pub const EPS: f64 = 1e-10;

///
/// 浮動小数点数のassert
///
#[macro_export]
macro_rules! assert_float {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((*left_val - *right_val).abs() < EPS) {
                    panic!(
                        "assertion failed: `(left == right)` (left: `{:?}`, right: `{:?})`",
                        left_val, right_val
                    );
                }
            }
        }
    };
}
