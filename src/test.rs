#[cfg(test)]
mod equality {
    use crate::Fraction;

    #[test]
    fn equality() {
        let half = Fraction::new(1, 2);
        let half_unsimplified = Fraction::new(2, 4);

        assert_eq!(half, half_unsimplified);
    }
}
#[cfg(test)]
mod ordering {
    use crate::Fraction;

    #[test]
    fn ordering() {
        let half = Fraction::new(1, 2);
        let half_unsimplified = Fraction::new(2, 4);
        let third = Fraction::new(1, 3);

        assert!(half == half_unsimplified);
        assert!(half != third);

        assert!(half > third);
        assert!(third < half);
    }
}
#[cfg(test)]
mod arithmetic_operations {
    use crate::Fraction;

    #[test]
    fn add() {
        let half = Fraction::new(1, 2);
        let fourth = Fraction::new(1, 4);

        assert_eq!((half + half), Fraction::new(1, 1));
        assert_eq!((half + -half), Fraction::ZERO);
        assert_eq!((half + fourth), Fraction::new(3, 4));
    }

    #[test]
    fn sub() {
        let half = Fraction::new(1, 2);
        let fourth = Fraction::new(1, 4);

        assert_eq!((half - half), Fraction::ZERO);
        assert_eq!((half - -half), Fraction::new(1, 1));
        assert_eq!((-half - half), Fraction::new(-1, 1));
        assert_eq!((half - fourth), Fraction::new(1, 4));
        assert_eq!((fourth - half), Fraction::new(-1, 4));
    }

    #[test]
    fn mul() {
        let half = Fraction::new(1, 2);
        let third = Fraction::new(1, 3);

        assert_eq!((half * third), Fraction::new(1, 6));
        assert_eq!((half * half), Fraction::new(1, 4));
        assert_eq!((half * -half), Fraction::new(-1, 4));
    }

    #[test]
    fn div() {
        let third = Fraction::new(1, 3);
        let half = Fraction::new(1, 2);

        assert_eq!((third / half), Fraction::new(2, 3));
        assert_eq!((half / third), Fraction::new(3, 2));

        assert_eq!((third / third), Fraction::ONE);
        assert_eq!((third / -third), -Fraction::ONE);
    }

    #[test]
    fn neg() {
        let half = Fraction::new(1, 2);
        assert_eq!(-half, Fraction::new(-1, 2));

        let zero = Fraction::ZERO;
        assert_eq!(-zero, zero);
    }
}
#[cfg(test)]
mod assignment {
    use crate::Fraction;

    #[test]
    fn add() {
        let mut frac = Fraction::new(1, 2);
        frac += Fraction::new(1, 3);
        frac += Fraction::ZERO;

        assert_eq!(frac, Fraction::new(5, 6));
    }

    #[test]
    fn sub() {
        let mut frac = Fraction::new(1, 2);
        frac -= Fraction::new(1, 3);
        frac -= Fraction::ZERO;

        assert_eq!(frac, Fraction::new(1, 6));
    }

    #[test]
    fn mul() {
        let mut frac = Fraction::new(1, 2);
        frac *= Fraction::new(3, 4);
        frac *= Fraction::ONE;
        assert_eq!(frac, Fraction::new(3, 8));
    }

    #[test]
    fn div() {
        let mut frac = Fraction::new(3, 4);
        frac /= Fraction::new(6, 4);
        frac /= Fraction::ONE;

        assert_eq!(frac, Fraction::new(1, 2));
    }
}

#[cfg(test)]
mod zero {
    use crate::Fraction;

    #[test]
    fn is_zero() {
        let manual_zero = Fraction::new(0, 1);
        assert_eq!(manual_zero, Fraction::ZERO);

        let zero_alternative = Fraction::new(0, 10);
        assert_eq!(zero_alternative, Fraction::ZERO);
    }

    #[test]
    fn zero_denominator() {
        let invalid = Fraction::try_new(1, 0);
        assert!(invalid.is_err());

        let invalid_negative = Fraction::try_new(1, -0);

        assert!(invalid_negative.is_err());

        let invalid_negative = Fraction::try_new(-1, 0);
        assert!(invalid_negative.is_err());

        let invalid_negative = Fraction::try_new(-1, -0);
        assert!(invalid_negative.is_err());
    }
}

#[cfg(test)]
mod simplification {
    use crate::Fraction;

    #[test]
    fn simplification() {
        let already_simplified = Fraction::new(3, 4);
        assert_eq!(already_simplified, Fraction::new(3, 4));

        let unsimplified = Fraction::new(4, 8);
        assert_eq!(unsimplified, Fraction::new(1, 2));

        let negative_unsimplified = Fraction::new(-6, -8);
        assert_eq!(negative_unsimplified, Fraction::new(3, 4));

        let negative = Fraction::new(-6, 8);
        assert_eq!(negative, Fraction::new(-3, 4));

        let reverse_negative = Fraction::new(6, -8);
        assert_eq!(reverse_negative, Fraction::new(-3, 4));
    }
}

#[cfg(test)]
mod negative_fractions {
    use crate::*;

    #[test]
    fn negative_fractions() {
        let half_negative = Fraction::new(-1, 2);
        assert_eq!(half_negative, Fraction::new(-1, 2));

        let half_negative_switched = Fraction::new(1, -2);
        assert_eq!(half_negative_switched, Fraction::new(-1, 2));

        let half_double_negative = Fraction::new(-1, -2);
        assert_eq!(half_double_negative, Fraction::new(1, 2));
    }
}

#[cfg(test)]
mod zero_numerator_tests {
    use crate::Fraction;

    #[test]
    fn zero_numerator() {
        let zero = Fraction::ZERO;
        let half = Fraction::new(1, 2);

        assert_eq!((zero + half), half);
        assert_eq!((zero - half), -half);
        assert_eq!((zero * half), zero);
        assert_eq!((zero / half), zero);
    }
}

#[cfg(test)]
mod reciprocal_tests {
    use crate::Fraction;

    #[test]
    fn reciprocal() {
        let half = Fraction::new(1, 2);
        let reciprocal_half = half.recip();

        assert_eq!(reciprocal_half, Fraction::new(2, 1));

        let negative_half = Fraction::new(-1, 2);
        let reciprocal_negative_half = negative_half.recip();

        assert_eq!(reciprocal_negative_half, Fraction::new(-2, 1));
    }
}

#[cfg(test)]
mod helper_functions_tests {
    use crate::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(101, 103), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 5), 20);
        assert_eq!(lcm(7, 3), 21);
        assert_eq!(lcm(21, 6), 42);
    }
}

#[cfg(test)]
mod display_tests {
    use crate::*;

    #[test]
    fn display() {
        let fraction = Fraction::new(1, 2);
        assert_eq!(format!("{}", fraction), "1/2");

        let negative = Fraction::new(-1, 2);
        assert_eq!(format!("{}", negative), "-1/2");

        let negative_switched = Fraction::new(1, -2);
        assert_eq!(format!("{}", negative_switched), "-1/2");

        let whole_number = Fraction::new(4, 2);
        assert_eq!(format!("{}", whole_number), "2");

        let zero = Fraction::ZERO;
        assert_eq!(format!("{}", zero), "0");
    }
}
