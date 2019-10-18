use std::fs::File;
use std::io::{prelude::*, Result};

/// [Coded triangle numbers](https://projecteuler.net/problem=42)
///
/// solve() returns number of triangle words in resource/words.txt
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p042::solve().unwrap(), 162);
/// ```
pub fn solve() -> Result<usize> {
    let mut file = File::open("resource/p042_words.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let num_triangles = buffer
        .split(',')
        .map(|w| w.trim_matches('"'))
        .filter(|w| is_triangle(w))
        .count();

    Ok(num_triangles)
}

fn is_triangle(word: &str) -> bool {
    let total = word.chars().map(|c| c as usize - 64).sum::<usize>();

    let mut n = 1;
    let mut counter = 2;

    while n < total {
        n += counter;
        counter += 1;
    }

    n == total
}
