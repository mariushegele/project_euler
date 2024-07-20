use largest_prime_factor::*;

fn main() {
    let number: u64 = std::env::args().nth(1).expect("no number given").parse().unwrap();

    let factor = largest_prime_factor(number);
    println!("The largest prime factor of {number} is {factor}");
}
