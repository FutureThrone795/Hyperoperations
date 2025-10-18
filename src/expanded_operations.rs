use crate::ExponentialNumber;





//flogn for negative inputs, fexpn for positive inputs
pub fn flogn_fexpn(n: i64, val: f64) -> Result<f64, String> {
    if n == 0 {
        return Ok(val);
    }
    if n < 0 {
        return flogn(n.unsigned_abs(), val);
    }
    return Ok(fexpn(n.unsigned_abs(), val));
}

//logn for negative inputs, expn for positive inputs
pub fn logn_expn(n: i64, val: ExponentialNumber) -> ExponentialNumber {
    if n == 0 {
        return val;
    }
    if n < 0 {
        return logn(n.unsigned_abs(), val);
    }
    return expn(n.unsigned_abs(), val);
}





pub fn in_flogn_domain(n: u64, val: f64) -> bool {
    return val > fexpn(n, 1.0);
}

pub fn flogn(n: u64, val: f64) -> Result<f64, String> {
    if !in_flogn_domain(n, val) {
        return Err(format!("Float is not within flogn^({}) domain: {}", n, val).to_string());
    }

    if n == 1 {
        return Ok(val.ln());
    }

    return Ok(flogn(n - 1, val).unwrap().ln());
}

pub fn logn(n: u64, val: ExponentialNumber) -> ExponentialNumber {
    return ExponentialNumber {internal_val: val.internal_val, k_index: val.k_index - n as i64};
}






pub fn fexpn(n: u64, val: f64) -> f64 {
    if n == 1 {
        return val.exp();
    }

    return fexpn(n - 1, val).exp();
}

pub fn expn(n: u64, val: ExponentialNumber) -> ExponentialNumber {
    return ExponentialNumber {internal_val: val.internal_val, k_index: val.k_index + n as i64};
}






pub fn dotn(n: i64, multiplier: ExponentialNumber, multiplicand: ExponentialNumber) -> Result<ExponentialNumber, String> { 
    if !multiplier.in_as_k_index_domain(n) {
        return Err(format!("Multiplier ExponentialNumber is not within dotn^({}) domain: {}", n, multiplier).to_string());
    }
    if !multiplicand.in_as_k_index_domain(n) {
        return Err(format!("Addend ExponentialNumber is not within dotn^({}) domain: {}", n, multiplicand).to_string());
    }

    //Errors should be handled by the above checks
    let multiplier_normalized = multiplier.as_k_index(n).unwrap();
    let addend_normalized = multiplicand.as_k_index(n).unwrap();
    //That being said, there could be a lot done here to improve accuracy

    return Ok(ExponentialNumber { internal_val: multiplier_normalized.internal_val * addend_normalized.internal_val, k_index: n })
}






pub fn plusn(n: i64, augend: ExponentialNumber, addend: ExponentialNumber) -> Result<ExponentialNumber, String> { 
    if !augend.in_as_k_index_domain(n) {
        return Err(format!("Augend ExponentialNumber is not within plusn^({}) domain: {}", n, augend).to_string());
    }
    if !addend.in_as_k_index_domain(n) {
        return Err(format!("Addend ExponentialNumber is not within plusn^({}) domain: {}", n, addend).to_string());
    }

    //Errors should be handled by the above checks
    let augend_normalized = augend.as_k_index(n).unwrap();
    let addend_normalized = addend.as_k_index(n).unwrap();
    //That being said, there could be a lot done here to improve accuracy

    return Ok(ExponentialNumber { internal_val: augend_normalized.internal_val + addend_normalized.internal_val, k_index: n })
}