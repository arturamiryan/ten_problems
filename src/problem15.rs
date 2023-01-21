use num_bigint::BigUint;
use num_bigint::ToBigUint;

// In fact this is permutation
// ((width + height)!) / (width!*height!)
pub fn problem15() {
    let width = 20;
    let height = 20;
    let routes = (factorial(width + height)) / (factorial(width) * factorial(height));
    println!("Problem 15: Routes at 20x20 grid: {}", routes);
}

fn factorial(num: u128) -> BigUint {
    match num {
        0 => 1_i32.to_biguint().unwrap(),
        1.. => (1..num + 1).product(),
    }
}
