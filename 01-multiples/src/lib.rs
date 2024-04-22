

pub fn sum_of_multiples_below(nums: &Vec<u32> , below: u32) -> u32 {
    let mut sum: u32 = 0;
    for i in 1..below {
        for j in nums {
            if i % j == 0 {
                sum += i;
                break;
            }
        }
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_multiples_below() {
        assert_eq!(sum_of_multiples_below(&vec![3, 5], 10), 23);
    }

}
