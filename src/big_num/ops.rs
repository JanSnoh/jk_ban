

use super::{BigNum, DECIMAL_DIGITS};

use std::ops::{Add, Mul, Sub, Div, AddAssign};


impl Mul for BigNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let coeff = self.coeff*rhs.coeff;
        let exp = self.exp + rhs.exp;
        
        BigNum::new(coeff, exp)
    }
}

impl Div for BigNum {
    type Output = Self;

    ///Division always returns a value larger or equal to one 
    ///and may therefore be larger than the true result.
    fn div(self, rhs: Self) -> Self::Output {
        if rhs>self {return BigNum::one();}
        let new_coeff = std::cmp::max(self.coeff/rhs.coeff,1);
        let new_exp = std::cmp::max(self.exp-rhs.exp,0);

        BigNum::new(new_coeff, new_exp)
    }
}

impl Add for BigNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (bigger, smaller) = if self>rhs {(self, rhs)}
            else {(rhs,self)};
        
        if bigger.exp>smaller.exp+DECIMAL_DIGITS as u128 {return bigger}

        let new_coeff = bigger.coeff*
            u128::pow(10, (bigger.exp-smaller.exp) as u32) + smaller.coeff;
        Self::new(new_coeff, smaller.exp)
    }
}

impl Sub for BigNum {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs>self{return None;}
        if self.exp>rhs.exp+DECIMAL_DIGITS as u128 {return Some(self)}
        let new_coeff = self.coeff*
            u128::pow(10, (self.exp-rhs.exp) as u32)-rhs.coeff;
            Some(Self::new(new_coeff, rhs.exp))
        }
}

impl BigNum {
    ///Gives 4 digits of precision in the result!
    pub fn pow(self, exponent:f64) -> Self{
        //Some of the +1's may be wrong, but hey, erstes Spiel.
        if self.is_zero() {return Self::zero()}
        let significant_digits = 4;
        let loggi = self.coeff.ilog10()+1;
        let shifted_magnitude = (loggi.saturating_sub(significant_digits+1)) as u128;
        if self.coeff.leading_zeros()>110 {let shifted_magnitude=0u128;}

        let old_coeff_float = (self.coeff/10u128.pow(shifted_magnitude as u32)) as f64;
        let new_coeff = old_coeff_float.powf(exponent) as u128;

        let new_exp = (self.exp+shifted_magnitude)*(exponent as u128);
        Self::new(new_coeff, new_exp)
    }

    pub fn increment(&mut self){
        if self.exp>0{return;}
        self.coeff+=1;
    }
}

impl PartialOrd for BigNum{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.exp.partial_cmp(&other.exp) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.coeff.partial_cmp(&other.coeff)
    }
}

impl AddAssign for BigNum{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}