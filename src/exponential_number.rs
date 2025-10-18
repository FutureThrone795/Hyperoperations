use std::fmt::Display;
use std::fmt::Formatter;

use crate::expanded_operations::in_flogn_domain;
use crate::{flogn, fexpn};

#[derive(Clone, Copy, Debug)]
pub struct ExponentialNumber {
    pub internal_val: f64,
    pub k_index: i64
}

impl Display for ExponentialNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        if self.k_index < 0 {
            return write!(f, "K({}): {:}", self.k_index, self.internal_val);
            
        }
        return write!(f, "K{}: {:}", self.k_index, self.internal_val);
    }
}

impl From<f64> for ExponentialNumber {
    fn from(f: f64) -> ExponentialNumber {
        return ExponentialNumber { internal_val: f, k_index: 0 }
    }
}

impl ExponentialNumber {
    pub fn in_as_k_index_domain(self, target_k: i64) -> bool {
        let diff: i64 = target_k - self.k_index;

        if diff > 0 {
            if !in_flogn_domain(diff as u64, self.internal_val) {
                return false;
            }
        }

        return true;
    }

    pub fn as_k_index(self, target_k: i64) -> Result<ExponentialNumber, String> {
        let diff: i64 = target_k - self.k_index;

        if !self.in_as_k_index_domain(target_k) {
            return Err(format!("ExponentialNumber with k_index {} cannot be expressed with target_k {}", self.k_index, target_k).to_string());
        }

        if diff == 0 {
            return Ok(self);
        }
        if diff > 0 {
            //Errors should be handled by the above self.in_as_k_index_domain(target_k) condition
            return Ok(ExponentialNumber { internal_val: flogn(diff as u64, self.internal_val).unwrap(), k_index: target_k })
        }

        // diff < 0
        return Ok(ExponentialNumber { internal_val: fexpn(diff.unsigned_abs(), self.internal_val), k_index: target_k })
    }
}