
pub fn sum_of_primes_below(n: u64) -> u64 {
    let mut sum: u64 = 2;
    let mut primes = vec![2];
    let mut candidate: u64 = 3;
    while *primes.last().unwrap() < n {
        if is_prime(candidate, &primes) {
            if candidate >= n {
                break;
            }
            primes.push(candidate);
            sum += candidate;
        }
        candidate += 1;
    }

    sum
}

fn is_prime(n: u64, smaller_primes: &Vec<u64>) -> bool {
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
    fn test_sum_of_primes_below() {
        assert_eq!(sum_of_primes_below(10), 17);
    }
}