fn sum_square_difference(n: u64) -> u64 {
    square_of_sum_up_to(n) - sum_of_squares_up_to(n)
}

fn sum_of_squares_up_to(n: u64) -> u64 {
    (1..=n).into_iter().map(|x| x.pow(2)).sum()
}

fn square_of_sum_up_to(n: u64) -> u64 {
    sum_up_to(n).pow(2)
}

fn sum_up_to(n: u64) -> u64 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares_up_to(4), 30);
        assert_eq!(sum_of_squares_up_to(10), 385);
    }

    #[test]
    fn test_sum_up_to() {
        assert_eq!(sum_up_to(1), 1);
        assert_eq!(sum_up_to(2), 3);
        assert_eq!(sum_up_to(5), 15);
    }

    #[test]
    fn test_square_of_sum() {
        assert_eq!(square_of_sum_up_to(10), 3025);
    }

    #[test]
    fn test_sum_square_difference() {
        assert_eq!(sum_square_difference(10), 2640);
        assert_eq!(sum_square_difference(100), 25164150);
    }
}