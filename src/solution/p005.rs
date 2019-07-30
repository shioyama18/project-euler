// Smallest Multiple
// https://projecteuler.net/problem=5

use crate::util::prime;

pub fn solve(n: usize) -> usize {
    let p = prime::Prime::new();
    p.take_while(|&x| x <= n)
        .map(|mut x| {
            let mult = x;
            while x * mult < n {
                x *= mult;
            }
            x
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(10), 2520);
        assert_eq!(solve(20), 232792560);
    }
}
