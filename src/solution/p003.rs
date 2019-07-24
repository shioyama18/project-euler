// Largest prime factor
// https://projecteuler.net/problem=3

use crate::util::prime::Prime;

pub fn solve(n: usize) -> usize {
    let prime_factors = Prime::prime_factors(n);
    *prime_factors.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(600_851_475_143), 6857);
    }
}
