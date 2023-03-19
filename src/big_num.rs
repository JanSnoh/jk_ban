#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt::{Display, write};


mod ops;

const DECIMAL_DIGITS: u32 = 16;

///BigNums are represented as a 16 digit base-10 integer times ten to the power of exp.
///They are always non-negative. And always represent Integers.
#[derive(Default, Clone, Copy, Debug, PartialEq, Ord, Eq)]
pub struct BigNum {
    coeff: u128,
    exp: u128
}

impl BigNum{
    pub fn new(coeff: u128, exp: u128) -> Self{
        let mut res = BigNum { coeff, exp };
        res.normalize();
        res
    }
    pub fn normalize(&mut self){
        if self.coeff == 0 {self.exp=0; return;}

        let coeff_len = (self.coeff.ilog10())+1;
        if coeff_len > DECIMAL_DIGITS {
            let extra_digits = coeff_len - DECIMAL_DIGITS;
            self.coeff /=  u128::pow(10,extra_digits);
            self.exp += extra_digits as u128;
        } else if coeff_len < DECIMAL_DIGITS {
            let missing_digits = DECIMAL_DIGITS - coeff_len;
            let missing_digits = std::cmp::min(missing_digits as u128,
                 self.exp) as u32; 
            self.coeff *=  u128::pow(10,missing_digits);
            self.exp -= missing_digits as u128;
        }
    }

}

impl BigNum{
    pub fn zero() -> Self {
        BigNum { coeff: 0, exp: 0 }
    }

    pub fn is_zero(&self) -> bool {
        self.coeff==0 && self.exp==0
    }
    pub fn one() -> Self {
        BigNum { coeff: 1, exp: 0 }
    }
}


impl Display for BigNum{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{}e{}", self.coeff, self.exp))
    }
}

impl<N> From<N> for BigNum
    where N: Into<u128> 
    {
        fn from(value: N) -> Self {
            Self::new(value.into(), 0)
    }
}

