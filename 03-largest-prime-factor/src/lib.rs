fn get_first_factor(n: u64) -> u64 {
    let sqrt_n = (n as f64).sqrt().floor() as u64;
    for i in 2..sqrt_n + 1 {
        if n % i == 0 {
            return i;
        }
    }
    return n;
}

fn is_prime(n: u64) -> bool {
    return get_first_factor(n) == n;
}

pub fn all_prime_factors(n: u64) -> Vec<u64> {
    let first_factor = get_first_factor(n);
    if first_factor == n {
        return vec![n];
    }

    let mut factors = all_prime_factors(first_factor);
    let rest = all_prime_factors(n / first_factor);
    factors.extend(rest);
    factors.sort();

    factors
}

pub fn largest_prime_factor(n: u64) -> u64 {
    *all_prime_factors(n).last().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let primes = vec![0, 1, 2, 3, 5, 7, 11, 13, 17, 19];
        let non_primes = vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20];

        for prime in primes {
            assert!(is_prime(prime), "not prime {prime}");
        }
        for non_prime in non_primes {
            assert!(!is_prime(non_prime), "not non-prime {non_prime}");
        }
    }

    #[test]
    fn test_all_prime_factors() {
        assert_eq!(all_prime_factors(13195), vec![5, 7, 13, 29]);
    }
    
    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(largest_prime_factor(13195), 29);
    }
}
