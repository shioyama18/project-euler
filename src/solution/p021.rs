// Amicable numbers
// https://projecteuler.net/problem=21

use crate::util::prime::Prime;
use std::collections::HashMap;

pub fn solve(n: usize) -> usize {
    (2..n)
        .filter(|&i| {
            let x = Prime::sum_proper_divisor(i);
            i != x && i == Prime::sum_proper_divisor(x)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(10000), 31626);
    }
}
