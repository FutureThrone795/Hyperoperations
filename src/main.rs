mod expanded_operations;
use expanded_operations::{logn, expn, dotn};

mod exponential_number;
use exponential_number::ExponentialNumber;

fn main() {
    let two = ExponentialNumber::from(2.0);
    let three = ExponentialNumber::from(3.0);

    let log_neg_two = logn(1, ExponentialNumber::from(-2.0));
    println!("{}", log_neg_two);

    print_dot(0, two, three);
    print_dot(1, two, three);
}

fn print_dot(n: i64, a: ExponentialNumber, b: ExponentialNumber) {
    match dotn(n, a, b) {
        Ok(val) => println!("dotn^({})({}, {}) = {}", n, a, b, val),
        Err(err) => println!("dotn^({})({}, {}) = ERROR: {}", n, a, b, err)
    }
}