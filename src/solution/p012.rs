// Highly divisible triangular number
// https://projecteuler.net/problem=12

use crate::util::prime::Prime;
use std::collections::HashMap;

struct TriangularNumber {
    curr: usize,
    diff: usize,
}

impl TriangularNumber {
    pub fn new() -> Self {
        TriangularNumber { curr: 1, diff: 2 }
    }
}

impl Iterator for TriangularNumber {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr = self.curr + self.diff;
        self.diff += 1;

        Some(curr)
    }
}

pub fn solve(n: usize) -> usize {
    let t = TriangularNumber::new();
    t.skip_while(|&i| num_factors(i) <= n).next().unwrap()
}

fn num_factors(n: usize) -> usize {
    let p = Prime::prime_factors(n);
    let mut dict: HashMap<usize, usize> = HashMap::new();

    p.iter().for_each(|&x| *dict.entry(x).or_insert(0) += 1);

    for i in dict.values_mut() {
        *i += 1;
    }

    dict.values().product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(5), 28);
        assert_eq!(solve(500), 76576500);
    }
}
