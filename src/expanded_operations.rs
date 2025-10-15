use crate::ExponentialNumber;

pub fn logn(n: i64, val: ExponentialNumber) -> ExponentialNumber {
    if val.k_index < 0 {
        return ExponentialNumber {internal_val: val.internal_val, k_index: val.k_index - 1};
    }
    

    if n == 1 {
        return ExponentialNumber {internal_val: val.internal_val.ln(), k_index: val.k_index};
    }
    let mut a = logn(n-1, val);
    a.internal_val = a.internal_val.ln();
    return a;
}
pub fn expn(n: i64, val: ExponentialNumber) -> ExponentialNumber {
    if n == 1 {
        return ExponentialNumber {internal_val: val.internal_val.exp(), k_index: val.k_index};
    }
    let mut a = expn(n-1, val);
    a.internal_val = a.internal_val.exp();
    return a;
}

pub fn dotn(n: i64, augend: ExponentialNumber,addend: ExponentialNumber) -> Result<ExponentialNumber, String> { 
    if !augend.in_dotn_domain(n) {
        return Err("The augend supplied into dotn was outside of its domain!".to_string());
    }
    if !addend.in_dotn_domain(n) {
        return Err("The addend supplied into dotn was outside of its domain!".to_string());
    }
    
    if n == 0 {
        return Ok(ExponentialNumber {internal_val: (augend.internal_val * addend.internal_val), k_index: 0});
    }

    let logn_augend = logn(n, augend);
    let logn_addend = logn(n, addend); 

    let logn_dot_logn = dotn(0, logn_augend, logn_addend);

    match logn_dot_logn {
        Err(_) => return Err("Logn dot logn failed to parse correctly".to_string()),
        _ => ()
    };

    return Ok(expn(n, logn_dot_logn.unwrap()));
}