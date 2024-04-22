use multiples::*;

fn main() {
    println!("Num. of multiples of 3 and 5 below 10: {}", sum_of_multiples_below(&vec![3, 5], 10));
    println!("Num. of multiples of 3 and 5 below 1000: {}", sum_of_multiples_below(&vec![3, 5], 1000));
}


