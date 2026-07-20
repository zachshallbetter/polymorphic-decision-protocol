use serde::{Serialize, Deserialize};
use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Cents(pub u64);

impl Cents {
    pub const ZERO: Self = Cents(0);

    pub fn from_dollars(dollars: f64) -> Self {
        Cents((dollars * 100.0).round() as u64)
    }

    pub fn to_dollars(&self) -> f64 {
        self.0 as f64 / 100.0
    }
}

impl Default for Cents {
    fn default() -> Self {
        Cents::ZERO
    }
}

impl std::fmt::Display for Cents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.to_dollars())
    }
}

impl Add for Cents {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Cents(self.0 + other.0)
    }
}

impl Sub for Cents {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Cents(self.0.saturating_sub(other.0))
    }
}

impl AddAssign for Cents {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl SubAssign for Cents {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.saturating_sub(other.0);
    }
}

// Support multiplying by standard float factors (e.g. fraction)
impl Mul<f64> for Cents {
    type Output = Self;
    fn mul(self, factor: f64) -> Self {
        if factor <= 0.0 {
            Cents::ZERO
        } else {
            Cents((self.0 as f64 * factor).round() as u64)
        }
    }
}

impl Div<f64> for Cents {
    type Output = Self;
    fn div(self, divisor: f64) -> Self {
        if divisor <= 0.0 {
            Cents::ZERO
        } else {
            Cents((self.0 as f64 / divisor).round() as u64)
        }
    }
}
