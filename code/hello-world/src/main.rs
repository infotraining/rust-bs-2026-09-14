use rand::{self, random};
use std::iter;

fn find_min<T: Ord>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }
    let mut min = &slice[0];
    for item in slice.iter().skip(1) {
        if item < min {
            min = item;
        }
    }
    Some(min)
}

fn find_max<T: Ord>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }
    let mut max = &slice[0];
    for item in slice.iter().skip(1) {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

#[test]
fn test_find_min() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    assert_eq!(find_min(&numbers), Some(&1));

    let empty: Vec<i32> = vec![];
    assert_eq!(find_min(&empty), None);
}

#[test]
fn test_find_max() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    assert_eq!(find_max(&numbers), Some(&9));

    let empty: Vec<i32> = vec![];
    assert_eq!(find_max(&empty), None);
}

struct Record<'a, T> {
    min_value: Option<&'a T>,
    max_value: Option<&'a T>,
}

fn find_min_max<'a, T: Ord>(slice: &'a [T]) -> Record<'a, T> {
    Record {
        min_value: find_min(slice),
        max_value: find_max(slice),
    }
}


fn main() {
    let mut message = "Hello, world!".to_string();
    message += "!!";
    println!("{message}");

    let numbers = iter::from_fn(|| Some(random::<u8>())).take(10).collect::<Vec<u8>>();
    let words: Vec<String> = vec!["one", "two", "three", "four", "five"].iter().map(|s| s.to_string()).collect();

    println!("Numbers: {:?}", numbers);
    println!("Words: {:?}", words);

    let min_number = find_min(&numbers);
    println!("Minimum number: {:?}", min_number);

    let max_number = find_max(&numbers);
    println!("Maximum number: {:?}", max_number);


    let min_max_numbers = find_min_max(&numbers);
    println!("Minimum and maximum numbers: {:?}", (min_max_numbers.min_value, min_max_numbers.max_value));

    match min_max_numbers {
        Record { min_value: Some(min), max_value: Some(max) } => {
            println!("Matched min and max numbers: min = {:?}, max = {:?}", min, max);
        }
        _ => {
            println!("No min and max numbers found");
        }
    }

    let min_word = find_min(&words);
    println!("Minimum word: {:?}", min_word);

    let max_word = find_max(&words);
    println!("Maximum word: {:?}", max_word);

    let min_max_words = find_min_max(&words);
    println!("Minimum and maximum words: {:?}", (min_max_words.min_value, min_max_words.max_value));

    match min_max_words {
        Record { min_value: Some(min), max_value: Some(max) } => {
            println!("Matched min and max words: min = {:?}, max = {:?}", min, max);
        }
        _ => {
            println!("No min and max words found");
        }
    }
}
