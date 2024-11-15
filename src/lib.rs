mod test;

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

pub static ZERO: Fraction = Fraction {
    numerator: 0,
    denominator: 1,
};

impl Fraction {
    pub fn try_new(numerator: i64, denominator: i64) -> Result<Self, FractionError> {
        if denominator == 0 {
            return Err(FractionError::ZeroDenominator);
        }
        Self {
            numerator,
            denominator,
        }
        .try_simplify()
    }

    pub fn new(numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("{}", FractionError::ZeroDenominator);
        }
        Self {
            numerator,
            denominator,
        }
        .simplify()
    }

    pub fn try_simplify(self) -> Result<Self, FractionError> {
        if self.denominator == 0 {
            return Err(FractionError::ZeroDenominator);
        }

        let gcd = checked_gcd(self.numerator, self.denominator)?;
        let mut numerator = self.numerator / gcd;
        let mut denominator = self.denominator / gcd;

        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }
    pub fn simplify(self) -> Self {
        if self.denominator == 0 {
            panic!("{}", FractionError::ZeroDenominator);
        }

        let gcd = gcd(self.numerator, self.denominator);
        let mut numerator = self.numerator / gcd;
        let mut denominator = self.denominator / gcd;

        // Reverse the sign if the denominator is negative
        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }

        Self {
            numerator,
            denominator,
        }
    }

    pub fn try_recip(self) -> Result<Self, FractionError> {
        Self::try_new(self.denominator, self.numerator)
    }

    pub fn recip(self) -> Self {
        Self::new(self.denominator, self.numerator)
    }
}

impl Add for Fraction {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let lcm = lcm(self.denominator, other.denominator);
        let numerator =
            self.numerator * (lcm / self.denominator) + other.numerator * (lcm / other.denominator);
        Fraction::new(numerator, lcm)
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self == other {
            true => Fraction::new(0, 1),
            false => self + -other,
        }
    }
}
impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let numerator = self.numerator * other.numerator;
        let denominator = self.denominator * other.denominator;
        Self::new(numerator, denominator)
    }
}

impl Div for Fraction {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: Self) -> Self {
        if other.denominator == 0 {
            panic!("{}", FractionError::ZeroDenominator);
        }

        self * other.recip()
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self {
        Self::try_new(-self.numerator, self.denominator).unwrap()
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other
    }
}

// Implement the Display trait for Fraction
impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

// Implement Eq and Ord to compare fractions' values
impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.numerator * other.denominator).cmp(&(other.numerator * self.denominator)))
    }
}

// Utility functions
pub fn checked_gcd(mut a: i64, mut b: i64) -> Result<i64, FractionError> {
    a = a.checked_abs().ok_or(FractionError::GcdError)?;
    b = b.checked_abs().ok_or(FractionError::GcdError)?;

    let max_iterations = 1000;
    let mut iterations = 0;

    while b != 0 {
        if iterations >= max_iterations {
            return Err(FractionError::GcdError);
        }

        let temp = b;
        b = a % b;
        a = temp;

        iterations += 1;
    }
    Ok(a)
}
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn checked_lcm(a: i64, b: i64) -> Result<i64, FractionError> {
    let gcd = checked_gcd(a, b)?;
    Ok((a * b).abs() / gcd)
}

pub fn lcm(a: i64, b: i64) -> i64 {
    let gcd = gcd(a, b);
    (a * b).abs() / gcd
}

// Define the FractionError type
#[derive(Debug)]
pub enum FractionError {
    ZeroDenominator,
    GcdError,
}

// Implement Display for FractionError
impl std::fmt::Display for FractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FractionError::ZeroDenominator => write!(f, "Denominator cannot be zero"),
            FractionError::GcdError => write!(f, "Error computing GCD"),
        }
    }
}

// Implement Error for FractionError
impl std::error::Error for FractionError {}
