///
/// cargo equip 検証用コード
///
/// .githook/pre-commit に cargo equip を記載している。
/// cargo equip に失敗したら commit も失敗する。
///
use geometry::point2f::Point2f;

fn main() {
    let point = Point2f::new(0.5, 0.6);

    println!("{}", point);
}
