
fn sum_of_proper_divisors(a: u32) -> u32 {
    (1..a).filter(|x| a % x == 0).sum()
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
        let divisors = vec![1,2,4,5,10,11,20,22,44,55,110];
        assert_eq!(sum_of_proper_divisors(220), divisors.into_iter().sum());
        let divisors = vec![1,2,4,71,142];
        assert_eq!(sum_of_proper_divisors(284), divisors.into_iter().sum());
    }

    #[test]
    fn test_amicable() {
        assert!(!is_amicable(6));
        assert!(!is_amicable(28));
        assert!(is_amicable(220));
        assert!(is_amicable(284));
    }
}