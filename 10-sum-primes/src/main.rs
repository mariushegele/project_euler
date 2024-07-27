use sum_primes::*;

fn main() {
    let n = 2000000;
    let sum = sum_of_primes_below(n);

    println!("Sum of all primes below {n} is {sum}");
}
