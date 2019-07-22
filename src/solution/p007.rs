// 10001st prime
// https://projecteuler.net/problem=7

use crate::util::prime;

pub fn solve(n: usize) -> usize {
    let mut p = prime::Prime::new();
    p.nth(n - 1)
        .expect("Please input a positive integer greater than 0")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(6), 13);
        assert_eq!(solve(10001), 104743);
    }
}
