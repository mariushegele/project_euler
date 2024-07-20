
use std::ops::Range;

pub fn smallest_multiple_of_numbers_in_range(min: u32, max: u32) -> u32 {
    let checks: Vec<u32> = filter_numbers_to_check(min, max);
    let mut candidate = max;
    loop {
        if divisible_by_all_in(candidate, &checks) {
            return candidate;
        }
        candidate += 1;
    }
}

fn filter_numbers_to_check(min: u32, max: u32) -> Vec<u32> {
    let mut numbers_to_check = vec![];
    for i in min..=max {
        let mut larger_number_is_divisible_by_this_number = false;
        for j in i+1..=max {
            if j % i == 0 {
                larger_number_is_divisible_by_this_number = true
            }
        }
        if !larger_number_is_divisible_by_this_number {
            numbers_to_check.push(i);
        }
    }

    numbers_to_check
}


fn divisible_by_all_in(candidate: u32, checks: &Vec<u32>) -> bool {
    for i in checks {
        if candidate % i != 0 {
            return false;
        } 
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_filter_numbers_to_check() {
        assert_eq!(filter_numbers_to_check(1, 10), vec![6, 7, 8, 9, 10]);
        assert_eq!(filter_numbers_to_check(1, 20),
            vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    }

    #[test]
    fn test_smallest_multiple() {
        assert_eq!(smallest_multiple_of_numbers_in_range(1, 10), 2520);
        assert_eq!(smallest_multiple_of_numbers_in_range(1, 20), 232792560);
    }
}