pub fn is_divisible_by_3(n: u32) -> bool {
    //n % 3 == 0

    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
        % 3
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_divisible_by_3() {
        assert!(is_divisible_by_3(9));
        assert!(!is_divisible_by_3(10));
        assert!(is_divisible_by_3(0));
        assert!(!is_divisible_by_3(1));
        assert!(is_divisible_by_3(25371));
    }
}
