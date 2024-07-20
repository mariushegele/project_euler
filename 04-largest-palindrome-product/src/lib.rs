fn reverse_sub_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn is_palindrome(n: u64) -> bool {
    let n_str: String = n.to_string();
    let mid = n_str.len() / 2;

    let first_half: String = n_str[0..mid].to_string();
    let second_half_rev: String;
    if n_str.len() % 2 == 0 {
        second_half_rev = reverse_sub_string(&n_str[mid..]);
    } else {
        second_half_rev = reverse_sub_string(&n_str[mid+1..])
    }

    first_half == second_half_rev
}

pub fn largest_palindrome_product(num_digits: u8) -> u64 {
    let max_n = u64::pow(10, num_digits as u32) - 1;

    let mut max_palindrome = 1;
    for i in (1..=max_n).rev() {
        for j in (1..=max_n).rev() {
            if is_palindrome(i * j) {
                max_palindrome = u64::max(max_palindrome, i * j);
            }
        }
    }

    max_palindrome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(1));
        assert!(is_palindrome(313));
        assert!(is_palindrome(3113));
        assert!(is_palindrome(31513));
        assert!(!is_palindrome(123));
        assert!(!is_palindrome(529));
    }


    #[test]
    fn test_largest_palindrome_product() {
        assert_eq!(largest_palindrome_product(2), 9009);
        assert_eq!(largest_palindrome_product(3), 906609);
    }
}