pub fn is_divisible_by_11(n: u32) -> bool {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .fold(0i32, |acc, (i, digit)| {
            if i % 2 == 0 {
                acc + digit as i32
            } else {
                acc - digit as i32
            }
        }) % 11
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_divisible_by_11() {
        assert!(is_divisible_by_11(11));
        assert!(!is_divisible_by_11(10));
        assert!(is_divisible_by_11(0));
        assert!(!is_divisible_by_11(1));
        assert!(is_divisible_by_11(121));
        assert!(is_divisible_by_11(25817));
        assert!(!is_divisible_by_11(25818));
    }
}