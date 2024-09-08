use abundant_sums::*;

fn main() {
    let list = non_abundant_summable_numbers(1, 28123);
    let sum: u32 = list.iter().sum();
    // println!("numbers that are not expressed as sum of two abundant numbers: {list:?}")
    println!("sum of numbers that are not expressed as sum of two abundant numbers: {sum}")
}
