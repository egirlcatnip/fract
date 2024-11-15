mod test;

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Fraction {
    numerator: u64,
    denominator: u64,
    sign: Sign,
}

impl Fraction {
    pub const ZERO: Fraction = Fraction {
        numerator: 0,
        denominator: 1,
        sign: Sign::Positive,
    };
    pub const ONE: Fraction = Fraction {
        numerator: 1,
        denominator: 1,
        sign: Sign::Positive,
    };

    pub fn new(numerator: i64, denominator: i64) -> Self {
        Self::try_new(numerator, denominator).expect("Denominator cannot be zero")
    }

    pub fn try_new(numerator: i64, denominator: i64) -> Result<Fraction, FractionError> {
        if denominator == 0 {
            return Err(FractionError::ZeroDenominator);
        }
        let sign = Self::match_sign(numerator, denominator);
        let numerator = numerator.unsigned_abs();
        let denominator = denominator.unsigned_abs();

        Ok(Self {
            numerator,
            denominator,
            sign,
        }
        .simplify())
    }

    pub const fn simplify(self) -> Self {
        let gcd = gcd(self.numerator, self.denominator);
        let numerator = self.numerator / gcd;
        let denominator = self.denominator / gcd;
        let sign = self.sign;

        if numerator == 0 {
            return Self::ZERO;
        }

        Self {
            numerator,
            denominator,
            sign,
        }
    }

    fn recip(self) -> Self {
        if self.numerator == 0 {
            panic!("{}", FractionError::ZeroDenominator);
        }
        Self {
            numerator: self.denominator,
            denominator: self.numerator,
            sign: self.sign,
        }
    }
    const fn combine_signs(sign1: Sign, sign2: Sign) -> Sign {
        match (sign1, sign2) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
            (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Sign::Negative,
        }
    }

    const fn match_sign(numerator: i64, denominator: i64) -> Sign {
        let sign1 = match numerator.is_negative() {
            true => Sign::Negative,
            false => Sign::Positive,
        };
        let sign2 = match denominator.is_negative() {
            true => Sign::Negative,
            false => Sign::Positive,
        };

        Fraction::combine_signs(sign1, sign2)
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcm = lcm(self.denominator, other.denominator);
        let numerator_self =
            self.sign_factor() * (self.numerator as i64) * (lcm / self.denominator) as i64;
        let numerator_other =
            other.sign_factor() * (other.numerator as i64) * (lcm / other.denominator) as i64;
        let numerator = numerator_self + numerator_other;

        Fraction::new(numerator, lcm as i64)
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let numerator = self.safe_mul(self.numerator, other.numerator);
        let denominator = self.safe_mul(self.denominator, other.denominator);

        let sign = Self::combine_signs(self.sign, other.sign);

        Fraction {
            numerator,
            denominator,
            sign,
        }
        .simplify()
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.numerator == 0 {
            panic!("{}", FractionError::ZeroDenominator);
        }

        let reciprocal = other.recip();
        let numerator = self.safe_mul(self.numerator, reciprocal.numerator);
        let denominator = self.safe_mul(self.denominator, reciprocal.denominator);

        let sign = match (self.sign, reciprocal.sign) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
            (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Sign::Negative,
        };

        Fraction {
            numerator,
            denominator,
            sign,
        }
        .simplify()
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self {
        if self.numerator == 0 {
            return Self::ZERO;
        }
        Self {
            numerator: self.numerator,
            denominator: self.denominator,
            sign: match self.sign {
                Sign::Positive => Sign::Negative,
                Sign::Negative => Sign::Positive,
            },
        }
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

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let simplified = self.simplify();
        let sign = match simplified.sign {
            Sign::Positive => "",
            Sign::Negative => "-",
        };

        if simplified.denominator == 1 {
            write!(f, "{}{}", sign, simplified.numerator)
        } else {
            write!(
                f,
                "{}{}/{}",
                sign, simplified.numerator, simplified.denominator
            )
        }
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs = (self.sign_factor() * self.numerator as i64) * other.denominator as i64;
        let rhs = (other.sign_factor() * other.numerator as i64) * self.denominator as i64;
        lhs.partial_cmp(&rhs)
    }
}

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

const fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

impl Fraction {
    fn sign_factor(&self) -> i64 {
        match self.sign {
            Sign::Positive => 1,
            Sign::Negative => -1,
        }
    }

    fn safe_mul(&self, a: u64, b: u64) -> u64 {
        a.checked_mul(b)
            .expect("Multiplication overflow in Fraction")
    }
}

#[derive(Debug)]
pub enum FractionError {
    ZeroDenominator,
}

impl fmt::Display for FractionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FractionError::ZeroDenominator => write!(f, "Denominator cannot be zero"),
        }
    }
}

impl std::error::Error for FractionError {}
