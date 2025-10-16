use crate::ExponentialNumber;






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

pub fn logn(n: i64, val: ExponentialNumber) -> ExponentialNumber {
    return ExponentialNumber {internal_val: val.internal_val, k_index: val.k_index - n};
}






pub fn fexpn(n: u64, val: f64) -> f64 {
    if n == 1 {
        return val.exp();
    }

    return fexpn(n - 1, val).exp();
}

pub fn expn(n: i64, val: ExponentialNumber) -> ExponentialNumber {
    return ExponentialNumber {internal_val: val.internal_val, k_index: val.k_index + n};
}






pub fn dotn(n: i64, augend: ExponentialNumber,addend: ExponentialNumber) -> Result<ExponentialNumber, String> { 
    if !augend.in_as_k_index_domain(n) {
        return Err(format!("Augend ExponentialNumber is not within dotn^({}) domain: {}", n, augend).to_string());
    }
    if !addend.in_as_k_index_domain(n) {
        return Err(format!("Addend ExponentialNumber is not within dotn^({}) domain: {}", n, addend).to_string());
    }

    //Errors should be handled by the above checks
    let augend_normalized = augend.as_k_index(n).unwrap();
    let addend_normalized = addend.as_k_index(n).unwrap();

    return Ok(ExponentialNumber { internal_val: augend_normalized.internal_val * addend_normalized.internal_val, k_index: n })
}