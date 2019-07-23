// Summation of primes
// https://projecteuler.net/problem=10
use crate::util::prime::Prime;

pub fn solve(limit: usize) -> usize {
    let p = Prime::new();
    p.take_while(|&x| x < limit).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(10), 17);
        assert_eq!(solve(2_000_000), 142913828922);
    }
}
