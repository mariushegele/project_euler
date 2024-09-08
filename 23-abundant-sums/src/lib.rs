use amicable_numbers::*;
use std::collections::hash_set::HashSet;


pub fn is_abundant(n: &u32) -> bool {
    sum_of_proper_divisors(*n) > *n
}

pub fn abundant_numbers<'a>(upto: &'a u32) -> impl Iterator<Item = u32> + 'a {
    (12..=*upto).filter(is_abundant)
}

pub fn non_abundant_summable_numbers(start: u32, end: u32) -> Vec<u32> {
    let relevant_abundant_numbers: Vec<u32> = abundant_numbers(&28123).collect();
    let relevant_abundant_numbers_set: HashSet<u32> = HashSet::from_iter(relevant_abundant_numbers.clone());

    let mut not_sum_of_two_abundant_numbers: Vec<u32> = vec![];
    for n in start..=end {

        let mut is_sum = false;
        for i in relevant_abundant_numbers.iter() {
            if *i > n / 2 {
                break;
            }    
            if relevant_abundant_numbers_set.contains(&(n - i)) {
                is_sum = true;
                break;
            }
        }
        if !is_sum {
            not_sum_of_two_abundant_numbers.push(n);
        }
    }
    not_sum_of_two_abundant_numbers
}


#[cfg(test)]
mod tests {
    use crate::{abundant_numbers, is_abundant, non_abundant_summable_numbers};


    #[test]
    fn test_is_abundant() {
        assert!(is_abundant(&12));
    }

    #[test]
    fn test_abundant_summable_numbers() {
        assert_eq!(non_abundant_summable_numbers(23, 25).len(), 2);
        assert_eq!(non_abundant_summable_numbers(28124, 28500).len(), 0);
    }
}