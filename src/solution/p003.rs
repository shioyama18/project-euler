// Largest prime factor
// https://projecteuler.net/problem=3

use crate::util::prime::Prime;

pub fn solve(mut n: usize) -> usize {
    let mut p = Prime::new();
    let mut prime_factors = Vec::new();

    while let Some(x) = p.next() {
        if x * x > n {
            prime_factors.push(n);
            break;
        } else if n % x == 0 {
            n /= x;
            prime_factors.push(x);
        }
    }

    *prime_factors.iter().max().expect("This will not be empty")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(600_851_475_143), 6857);
    }
}
