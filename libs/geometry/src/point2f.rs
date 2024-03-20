// use proconio::derive_readable;

use super::angle::*;
use super::prelude::*;

/// 座標, ベクトル
// #[derive_readable]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point2f {
    pub x: f64,
    pub y: f64,
}

impl std::ops::Add for Point2f {
    type Output = Point2f;

    fn add(self, rhs: Self) -> Self {
        Point2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point2f {
    type Output = Point2f;

    fn sub(self, rhs: Self) -> Self {
        Point2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f64> for Point2f {
    type Output = Point2f;

    fn mul(self, k: f64) -> Self {
        Point2f {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

impl std::ops::Div<f64> for Point2f {
    type Output = Point2f;

    fn div(self, k: f64) -> Self {
        Point2f {
            x: self.x / k,
            y: self.y / k,
        }
    }
}

impl std::fmt::Display for Point2f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl Point2f {
    /// 原点、零ベクトル
    pub const ZERO: Point2f = Point2f { x: 0., y: 0. };

    /// 単位ベクトル
    pub const ONE: Point2f = Point2f { x: 1., y: 1. };

    pub fn new(x: f64, y: f64) -> Point2f {
        Point2f { x, y }
    }

    /// ノルム
    /// ベクトルの大きさ
    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// ノルム
    /// ベクトルの大きさ
    pub fn abs(&self) -> f64 {
        self.norm().sqrt()
    }

    /// マンハッタン距離
    /// 2点間の距離
    /// ```
    /// use asakuchi_geometry::prelude::*;
    ///
    /// use asakuchi_geometry::point2f::Point2f;
    ///
    /// let p1 = Point2f { x: 3., y: 0. };
    /// let p2 = Point2f { x: 0., y: 4. };
    ///
    /// assert!((p1.manhattan_distance(p2) - 7.0).abs() < EPS)
    /// ```
    pub fn manhattan_distance(&self, rhs: Point2f) -> f64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    /// ユークリッド距離
    /// 2点間の距離
    /// ```
    /// use asakuchi_geometry::prelude::*;
    ///
    /// use asakuchi_geometry::point2f::Point2f;
    ///
    /// let p1 = Point2f { x: 3., y: 0. };
    /// let p2 = Point2f { x: 0., y: 4. };
    ///
    /// assert!((p1.euclidean_distance(p2) - 5.0).abs() < EPS);
    /// ```
    pub fn euclidean_distance(&self, rhs: Point2f) -> f64 {
        ((self.x - rhs.x).powf(2.) + (self.y - rhs.y).powf(2.)).sqrt()
    }

    /// 内積
    pub fn dot(&self, rhs: Point2f) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// 外積
    pub fn cross(&self, rhs: Point2f) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }

    /// 2つのベクトルが直交するか
    ///
    /// 内積0なら直交している
    pub fn is_orthogonal(&self, rhs: Point2f) -> bool {
        self.dot(rhs).abs() < EPS
    }

    /// 2つのベクトルが平行か
    ///
    /// 外積0なら平行
    pub fn is_parallel(&self, rhs: Point2f) -> bool {
        self.cross(rhs).abs() < EPS
    }

    // ///
    // /// ベクトルa,bの位置関係
    // ///
    // #[allow(dead_code)]
    // fn counter_clockwise(p0: Point2f, p1: Point2f, p2: Point2f) -> CcwPattern {
    //     let a = p1 - p0;
    //     let b = p2 - p0;

    //     if a.cross(b) > EPS {
    //         return CcwPattern::CounterClockwise;
    //     }

    //     if a.cross(b) < -EPS {
    //         return CcwPattern::Clockwise;
    //     }

    //     if a.dot(b) < -EPS {
    //         return CcwPattern::OnlineBack;
    //     }

    //     if a.norm() < b.norm() {
    //         return CcwPattern::OnlineFront;
    //     }

    //     CcwPattern::OnSegment
    // }

    // /// 線分 p1p2 と線分 p3p4 の交差判定
    // #[allow(dead_code)]
    // fn intersect(p1: Point2f, p2: Point2f, p3: Point2f, p4: Point2f) -> bool {
    //     Self::counter_clockwise(p1, p2, p3) as i32 * Self::counter_clockwise(p1, p2, p4) as i32 <= 0
    //         && Self::counter_clockwise(p3, p4, p1) as i32
    //             * Self::counter_clockwise(p3, p4, p2) as i32
    //             <= 0
    // }

    // /// 角度
    // #[allow(dead_code)]
    // fn arg(&self) -> Radian {
    //     Radian(self.y.atan2(self.x))
    // }

    // /// 極座標から変換
    // #[allow(dead_code)]
    // fn poloar(r: f64, theta: Radian) -> Point2f {
    //     Point2f {
    //         x: theta.0.cos() * r,
    //         y: theta.0.sin() * r,
    //     }
    // }

    ///
    /// 任意点周りの回転移動（アフィン変換）
    ///
    /// https://imagingsolution.net/math/rotate-around-point/
    pub fn rotate(&self, center: Point2f, angle: Radian) -> Point2f {
        Point2f {
            x: self.x * angle.0.cos() - self.y * angle.0.sin() + center.x
                - center.x * angle.0.cos()
                + center.y * angle.0.sin(),
            y: self.x * angle.0.sin() + self.y * angle.0.cos() + center.y
                - center.x * angle.0.sin()
                - center.y * angle.0.cos(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::assert_float;

    // use super::*;

    #[test]
    fn geometry_works() {
        // todo
    }
}
