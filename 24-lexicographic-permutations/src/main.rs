use lexicographic_permutations::*;

fn main() {
    let elements: Vec<u32> = (0..=9).collect();
    let n = 1_000_000;
    let result = nth_lexicographic_permutation(&elements, n);
    println!("The {n}th lexicographic permutation of {elements:?} is {result:?}.");
}
