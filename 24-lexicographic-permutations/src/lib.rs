pub fn nth_lexicographic_permutation(digits: &Vec<u32>, n: usize) -> Vec<u32> {
    assert!(n >= 1);
    let permutations = sorted_permutations(digits);
    permutations[n - 1].clone()
}

fn sorted_permutations<T: Clone + PartialOrd>(elements: &Vec<T>) -> Vec<Vec<T>> {
    assert!(elements.len() > 0);
    assert!(is_sorted(elements));
    if elements.len() == 1 {
        return vec![elements.clone()];
    }
    let mut permutations = vec![];
    for (i, element) in elements.into_iter().enumerate() {
        let mut rest = elements.clone();
        rest.remove(i);

        let rest_permutations = sorted_permutations(&rest);
        for permutation in rest_permutations {
            let mut permutation_with_element = permutation.clone();
            permutation_with_element.insert(0, element.clone());
            permutations.push(permutation_with_element);
        }
    }

    permutations
}

fn is_sorted<T: PartialOrd>(elements: &Vec<T>) -> bool {
    let mut max: Option<&T> = None;
    for element in elements {
        match max {
            None => (),
            Some(i) => {
                if element < i {
                    return false
                }
            }
        }
        max = Some(element);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_lex_permutation() {
        assert_eq!(nth_lexicographic_permutation(&vec![0, 1, 2], 1), vec![0, 1, 2]);
        assert_eq!(nth_lexicographic_permutation(&vec![0, 1, 2], 2), vec![0, 2, 1]);
        assert_eq!(nth_lexicographic_permutation(&vec![0, 1, 2], 4), vec![1, 2, 0]);
        assert_eq!(nth_lexicographic_permutation(&vec![0, 1, 2], 6), vec![2, 1, 0]);
    }

    #[test]
    fn test_permutations() {
        assert_eq!(sorted_permutations(&vec![0, 1, 2]), vec![
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ]);
    }
}
