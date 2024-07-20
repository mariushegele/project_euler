use nth_prime::*;

fn main() {
    let n: u32 = std::env::args().nth(1).expect("no number given").parse().unwrap();
    
    let nth_prime = nth_prime(n);
    println!("The {n}-th prime is {nth_prime}")
}
