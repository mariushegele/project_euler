


pub fn sum_even_fib_below(below: u32) -> u32 {
    let mut i: u32 = 1;
    let mut j: u32 = 2;
    let mut k: u32;
    let mut sum: u32 = 2;

    loop {
        k = i + j;
        // println!("i: {i} j: {j} k: {k}");
        if k >= below {
            break;
        }

        if k % 2 == 0 {
            sum += k;
        }
        i = j;
        j = k;
    }
    
    sum
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sum_even_fib_below() {
        assert_eq!(sum_even_fib_below(3), 2);
        assert_eq!(sum_even_fib_below(100), 2+8+34);
    }

}
