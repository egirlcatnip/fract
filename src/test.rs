#[cfg(test)]
mod equality_tests {
    use crate::Fraction;

    #[test]
    fn equality() {
        let half = Fraction::new(1, 2);
        let half_alternate = Fraction::new(2, 4);

        assert_eq!(half, half_alternate);

        let third = Fraction::new(1, 3);
        assert_ne!(half, third);
    }
}
#[cfg(test)]
mod ordering_tests {
    use crate::Fraction;

    #[test]
    fn ordering() {
        let half = Fraction::new(1, 2);
        let half_alternate = Fraction::new(2, 4);
        let third = Fraction::new(1, 3);

        assert!(half == half_alternate);
        assert!(half != third);

        assert!(half > third);
        assert!(third < half);
    }
}
#[cfg(test)]
mod arithmetic_tests {
    use crate::Fraction;

    #[test]
    fn add() {
        let half = Fraction::new(1, 2);
        let fourth = Fraction::new(1, 4);

        assert_eq!((half + half), Fraction::new(1, 1));
        assert_eq!((half + -half), Fraction::new(0, 1));
        assert_eq!((half + fourth), Fraction::new(3, 4));
    }

    #[test]
    fn sub() {
        let half = Fraction::new(1, 2);
        let fourth = Fraction::new(1, 4);

        assert_eq!((half - half), Fraction::new(0, 1));
        assert_eq!((half - fourth), Fraction::new(1, 4));
        assert_eq!((fourth - half), Fraction::new(-1, 4));
    }

    #[test]
    fn mul() {
        let half = Fraction::new(1, 2);
        let third = Fraction::new(1, 3);
        let one = Fraction::new(1, 1);

        assert_eq!((half * third), Fraction::new(1, 6));
        assert_eq!((half * half), Fraction::new(1, 4));
        assert_eq!((half * -half), Fraction::new(-1, 4));
        assert_eq!((half * one), Fraction::new(1, 2));
    }

    #[test]
    fn div() {
        let half = Fraction::new(1, 2);
        let third = Fraction::new(1, 3);
        let one = Fraction::new(1, 1);

        assert_eq!((half / third), Fraction::new(3, 2));
        assert_eq!((half / half), Fraction::new(1, 1));
        assert_eq!((half / -half), Fraction::new(-1, 1));
        assert_eq!((half / one), Fraction::new(1, 2));
    }
}
#[cfg(test)]
mod assignment_tests {
    use crate::Fraction;

    #[test]
    fn add_assign() {
        let mut frac = Fraction::new(1, 2);
        frac += Fraction::new(1, 3);
        assert_eq!(frac, Fraction::new(5, 6));
    }

    #[test]
    fn sub_assign() {
        let mut frac = Fraction::new(1, 2);
        frac -= Fraction::new(1, 3);
        assert_eq!(frac, Fraction::new(1, 6));
    }

    #[test]
    fn mul_assign() {
        let mut frac = Fraction::new(1, 2);
        frac *= Fraction::new(3, 4);
        assert_eq!(frac, Fraction::new(3, 8));
    }

    #[test]
    fn div_assign() {
        let mut frac = Fraction::new(3, 4);
        frac /= Fraction::new(6, 4);
        assert_eq!(frac, Fraction::new(1, 2));
    }

    #[test]
    fn neg_assign() {
        let mut frac = Fraction::new(3, 4);
        frac = -frac;
        assert_eq!(frac, Fraction::new(-3, 4));
    }
}

#[cfg(test)]
mod zero_denominator_tests {
    use crate::Fraction;

    #[test]
    fn zero_denominator() {
        let invalid_fraction = Fraction::new_checked(1, 0);
        assert!(invalid_fraction.is_err());

        let invalid_negative_fraction = Fraction::new_checked(1, -0);
        assert!(invalid_negative_fraction.is_err());
    }
}

#[cfg(test)]
mod zero_tests {
    use crate::Fraction;

    #[test]
    fn zero() {
        let manual_zero = Fraction::new(0, 1);
        assert_eq!(manual_zero, Fraction::ZERO);

        let zero_alternative = Fraction::new(0, 10);
        assert_eq!(zero_alternative, Fraction::ZERO);
    }
}

#[cfg(test)]
mod simplification_tests {
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
mod negative_fractions_tests {
    use crate::*;

    #[test]
    fn negative_fractions() {
        let half = Fraction::new(1, 2);
        let half_negative = Fraction::new(-1, 2);
        assert_eq!(half_negative, Fraction::new(-1, 2));

        let half_double_negative = Fraction::new(-1, -2);
        assert_eq!(half_negative, Fraction::new(-1, 2));
        assert_eq!(half, half_double_negative);

        let minus_one = Fraction::new(-1, 1);
        let half_negative_alt = half * minus_one;
        assert_eq!(half_negative, half_negative_alt);

        let half_negative_switched = Fraction::new(1, -2);
        assert_eq!(half_negative_switched, Fraction::new(-1, 2));
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
    }
}
