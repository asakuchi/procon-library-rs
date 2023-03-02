use proconio::derive_readable;

/// 度(度数法)
#[derive_readable]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Degree(pub f64);

/// ラジアン(弧度法)
#[derive_readable]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Radian(pub f64);

/////////////////////////////////////////////////////

impl std::ops::Add for Degree {
    type Output = Degree;

    fn add(self, rhs: Self) -> Self {
        Degree(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Degree {
    type Output = Degree;

    fn sub(self, rhs: Self) -> Self {
        Degree(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Degree {
    type Output = Degree;

    fn mul(self, k: f64) -> Self {
        Degree(self.0 * k)
    }
}

impl std::ops::Div<f64> for Degree {
    type Output = Degree;

    fn div(self, k: f64) -> Self {
        Degree(self.0 / k)
    }
}

/////////////////////////////////////////////////////

impl std::ops::Add for Radian {
    type Output = Radian;

    fn add(self, rhs: Self) -> Self {
        Radian(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Radian {
    type Output = Radian;

    fn sub(self, rhs: Self) -> Self {
        Radian(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Radian {
    type Output = Radian;

    fn mul(self, k: f64) -> Self {
        Radian(self.0 * k)
    }
}

impl std::ops::Div<f64> for Radian {
    type Output = Radian;

    fn div(self, k: f64) -> Self {
        Radian(self.0 / k)
    }
}

/////////////////////////////////////////////////////

trait ToDegree {
    fn to_degree(&self) -> Degree;
}

impl ToDegree for f64 {
    fn to_degree(&self) -> Degree {
        Degree(*self)
    }
}

impl ToDegree for Radian {
    fn to_degree(&self) -> Degree {
        Degree(self.0 * 180. / std::f64::consts::PI)
    }
}

/////////////////////////////////////////////////////

pub trait ToRadian {
    fn to_radian(&self) -> Radian;
}

impl ToRadian for f64 {
    fn to_radian(&self) -> Radian {
        Radian(*self)
    }
}

impl ToRadian for Degree {
    fn to_radian(&self) -> Radian {
        Radian(self.0 * std::f64::consts::PI / 180.)
    }
}
