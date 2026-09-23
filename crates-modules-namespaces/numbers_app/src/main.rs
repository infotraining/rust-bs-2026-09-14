use numbers::prelude::*;

fn main() {
    println!("First 50 prime numbers:");
    for prime in primes_sequence().take(50) {
        println!("Prime: {}", prime);
    }

    println!("-------------------");

    println!("First 20 Fibonacci numbers:");
    for fib in fibonacci_sequence().take(20) {
        println!("Fibonacci: {}", fib);
    }

    println!("-------------------");

    println!("Divisibility checks:");
    let number = 356;
    println!("Is {} divisible by 2? {}", number, is_divisible_by_2(number));
    println!("Is {} divisible by 3? {}", number, is_divisible_by_3(number));
    println!("Is {} divisible by 11? {}", number, is_divisible_by_11(number));
    println!("Is {} divisible by 2^2? {}", number, is_divisible_by_power_of_2(number, 2));
}
