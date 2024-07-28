
pub fn factorial_digit_sum(num: u32) -> u32 {
    let mut factorial: Vec<u8> = vec![6];
    for i in 4..num {
        let i: Vec<u8> = i.to_string().chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect();
        factorial = symbolic_mul(&factorial, &i);
    }

    factorial.iter().map(|n| *n as u32).sum()
}

fn symbolic_mul(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let max_result_len = a.len() + b.len();
    let mut result_rev: Vec<u8> = vec![0; max_result_len];
    let mut a_padded_rev: Vec<u8> = vec![0; max_result_len];
    let mut b_padded_rev: Vec<u8> = vec![0; max_result_len];

    for (rev_index, num) in a.iter().rev().enumerate() {
        a_padded_rev[rev_index] = *num;
    }
    for (rev_index, num) in b.iter().rev().enumerate() {
        b_padded_rev[rev_index] = *num;
    }

    for i in 0..a.len() {
        for j in 0..b.len() {
            let m = a_padded_rev[i] * b_padded_rev[j];
            let mut k = 0;
            let mut carry = m;

            while carry > 0 {
                let new = carry + result_rev[i+j+k];
                let entry = new % 10;
                result_rev[i+j+k] = entry;
                carry = new / 10;
                k += 1;
            }
        }
    }

    while result_rev.last() == Some(&0) {
        result_rev.pop();
    }
    result_rev.reverse();
    result_rev
}

fn symbolic_mul_with_nums(a: u64, b: u64) -> u64 {
    let a = a.to_string().chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect();
    let b = b.to_string().chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect();

    let result = symbolic_mul(&a, &b);


    let characters = String::from_iter(result.iter().map(|i| char::from_digit(*i as u32, 10).unwrap()));
    println!("{a:?} * {b:?} = {result:?} = {characters:?}");
    characters.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_symbolic_mul() {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0..10000);
        let b = rng.gen_range(0..10000);
        assert_eq!(symbolic_mul_with_nums(a, b), a*b);
        assert_eq!(symbolic_mul_with_nums(a, b), a*b);
    }

    #[test]
    fn test_factorial_digit_sum() {
        assert_eq!(factorial_digit_sum(10), 27);
    }
}