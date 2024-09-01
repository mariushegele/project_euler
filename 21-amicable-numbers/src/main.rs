
use amicable_numbers::*;

fn main() {
    let n = 10000;
    let numbers: Vec<u32> = amicable_numbers(&n).collect();
    let sum: u32 = numbers.iter().sum();
    println!("The amicable numbers below {n} are {numbers:?}");

    println!("The sum of amicable numbers below {n} is {sum}");
}
