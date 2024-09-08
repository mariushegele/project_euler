pub fn proper_divisors<'a>(n: &'a u32) -> impl Iterator<Item = u32> + 'a {
    (1..=n/2).filter(|x| *n % x == 0)
}

pub fn sum_of_proper_divisors(n: u32) -> u32 {
    proper_divisors(&n).sum()
}

fn is_amicable(n: u32) -> bool {
    let a = sum_of_proper_divisors(n);
    a != n && sum_of_proper_divisors(a) == n
}

pub fn amicable_numbers<'a>(n: &'a u32) -> impl Iterator<Item = u32> + 'a {
    (1..*n).filter(|i| is_amicable(*i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_proper_divisors() {
        let expected_divisors = vec![1,2,4,5,10,11,20,22,44,55,110];
        let divisors: Vec<u32> = proper_divisors(&220).collect();
        assert_eq!(divisors, expected_divisors);
        assert_eq!(sum_of_proper_divisors(220), expected_divisors.into_iter().sum());

        let expected_divisors = vec![1,2,4,71,142];
        assert_eq!(sum_of_proper_divisors(284), expected_divisors.into_iter().sum());
    }

    #[test]
    fn test_amicable() {
        assert!(!is_amicable(6));
        assert!(!is_amicable(28));
        assert!(is_amicable(220));
        assert!(is_amicable(284));
    }
}