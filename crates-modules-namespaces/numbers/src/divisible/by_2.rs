/// Checks if a number is divisible by 2.
/// # Arguments
/// * `n` - The number to check.
/// # Examples
/// ```
/// use numbers::divisible::by_2::is_divisible_by_2;
/// assert!(is_divisible_by_2(4));
/// assert!(!is_divisible_by_2(3));
/// ```
pub fn is_divisible_by_2(n: u32) -> bool {
    n % 2 == 0
}

/// Checks if a number is divisible by 2 raised to the given power.
/// # Arguments
/// * `n` - The number to check.
/// * `power` - The power of 2 to check divisibility against.
/// # Examples
/// ```
/// use numbers::divisible::by_2::is_divisible_by_power_of_2;
/// assert!(is_divisible_by_power_of_2(8, 3));
/// assert!(!is_divisible_by_power_of_2(8, 4));
/// ```
pub fn is_divisible_by_power_of_2(n: u32, power: u32) -> bool {
    n % (1 << power) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_divisible_by_2() {
        assert!(is_divisible_by_2(4));
        assert!(!is_divisible_by_2(3));
        assert!(is_divisible_by_2(0));
        assert!(!is_divisible_by_2(1));
    }

    #[test]
    fn test_is_divisible_by_power_of_2() {
        assert!(is_divisible_by_power_of_2(8, 3));
        assert!(!is_divisible_by_power_of_2(8, 4));
        assert!(is_divisible_by_power_of_2(356, 2));
        assert!(!is_divisible_by_power_of_2(357, 2));
    }
}