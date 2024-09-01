fn alphabetical_value(c: char) -> usize {
    assert!(c.is_ascii_alphabetic() && c.is_uppercase());
    let upper_case_a_ascii = 0x41;
    (c as usize) - upper_case_a_ascii + 1

}

pub fn alphabetical_sum(name: &str) -> usize {
    name.chars().map(|c| alphabetical_value(c)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabetical_value() {

        assert_eq!(alphabetical_value('A'), 1);
        assert_eq!(alphabetical_value('Z'), 26);
    }

    #[test]
    fn test_score() {
        let name = "COLIN";

        assert_eq!(alphabetical_sum(name), 53);
    }
}