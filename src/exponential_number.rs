use std::fmt::Display;
use std::fmt::Formatter;

use crate::expn;

#[derive(Clone, Copy, Debug)]
pub struct ExponentialNumber {
    pub internal_val: f64,
    pub k_index: i64
}

impl Display for ExponentialNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "K{}: {}", self.k_index, self.internal_val)
    }
}

impl From<f64> for ExponentialNumber {
    fn from(f: f64) -> ExponentialNumber {
        return ExponentialNumber { internal_val: f, k_index: 0 }
    }
}

impl ExponentialNumber {
    pub fn in_dotn_domain(self, n: i64) -> bool {
        if n == 0 {
            return self.k_index == 0;
        }

        if n > 0 {
            return (self.k_index == 0) && (self.internal_val > expn(n, ExponentialNumber {internal_val: 1.0, k_index: 0}).internal_val);
        }

        if n < 0 {
            return self.k_index < n;
        }

        return false; //Code will never get here
    }    
}