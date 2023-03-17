//!
//! Introducing BAN: Big     Number. Brought to you by Kajus & Jan
//! To suit all your Idle Game needs ( or whatever else you need these ridiculously large numbers for)
//! A BigNum can get VERY large. I would write down just how large, but python crashed trying to calculate it.
//! The Maximum BigNum is approximately: (1**16)*10**(2**(128))
//! 

mod big_num; 

pub use big_num::BigNum;

#[cfg(test)]
mod tests {
    use num::One;

    use super::*;

    #[test]
    fn addition() {
        let lhs= BigNum::new(5, 0);
        let rhs= BigNum::new(20, 0);
        let expected= BigNum::new(25, 0);

        assert_eq!(lhs+rhs, expected);
    }

    #[test]
    fn addition_big() {
        let lhs= BigNum::new(5, 50);
        let rhs= BigNum::new(7, 51);
        let expected= BigNum::new(75, 50);

        assert_eq!(lhs+rhs, expected);
    }
    #[test]
    fn addition_large_diff_approx() {
        let lhs= BigNum::new(5, 50);
        let rhs= BigNum::new(7, 6);
        let expected= lhs.clone();

        assert_eq!(lhs+rhs, expected);
    }

    #[test]
    fn test_from() {
        let literal:u128 = 50000;
        let constructed = BigNum::new(5, 4);

        assert_eq!(constructed, BigNum::from(literal));
    }
    
    #[test]
    fn mult() {
        let lhs= BigNum::new(5, 50);
        let rhs= BigNum::new(7, 6);
        let expected= BigNum::new(35, 56);

        assert_eq!(lhs*rhs, expected);
    }

    #[test]
    fn diff() {
        let lhs= BigNum::new(4, 50);
        let rhs= BigNum::new(2, 5);
        let expected= BigNum::new(2, 45);

        assert_eq!(lhs/lhs, BigNum::one());
        assert_eq!(lhs/rhs, expected);
        assert_eq!(rhs/lhs, BigNum::one())
    }
    #[test]
    fn exponentiation() {
        let lhs= BigNum::new(5, 50);
        let exponent = 5.0;
        let expected= BigNum::new(3125, 250);

        assert_eq!(lhs.pow(exponent), expected);
    }

}
