pub fn nth_prime(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut candidate = 3;
    while primes.len() < n as usize {
        if is_prime(candidate, &primes) {
            primes.push(candidate);
        }
        candidate += 1;
    }

    *primes.last().unwrap()
}

fn is_prime(n: u32, smaller_primes: &Vec<u32>) -> bool {
    for p in smaller_primes {
        if n % p == 0 {
            return false;
        }
    } 

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(6), 13);
    }
}
