/// Crate for various number-related utilities.
/// Includes functions for prime number checking and Fibonacci number generation.
/// Modules:
/// - `primes`: Functions related to prime numbers.
/// - `fibonacci`: Functions related to Fibonacci numbers.
/// # Examples:
/// ```
/// use numbers::prelude::*;
/// assert!(is_prime(13));
/// let mut fib_iter = fibonacci_sequence();
/// assert_eq!(fib_iter.next(), Some(0));
/// assert_eq!(fib_iter.next(), Some(1));
/// assert_eq!(fib_iter.next(), Some(1));
/// assert_eq!(fib_iter.next(), Some(2));
/// ```

pub mod primes;
pub mod fibonacci;
pub mod divisible;

pub mod prelude {
    pub use crate::primes::is_prime;
    pub use crate::primes::generators::primes_sequence;
    pub use crate::fibonacci::generators::fibonacci_sequence;
    pub use crate::divisible::by_2::is_divisible_by_2;
    pub use crate::divisible::by_11::is_divisible_by_11;
    pub use crate::divisible::by_3::is_divisible_by_3;
    pub use crate::divisible::by_2::is_divisible_by_power_of_2;
}