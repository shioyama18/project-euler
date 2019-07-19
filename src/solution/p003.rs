// Largest prime factor
// https://projecteuler.net/problem=3

use crate::util::prime::Prime;

pub fn solve(n: usize) -> usize {
    let p = Prime::new();

    p.take_while(|x| x * x < n)
        .filter(|x| n % x == 0)
        .max()
        .unwrap_or(n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(600_851_475_143), 6857);
    }
}
