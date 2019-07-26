// Power digit sum
// https://projecteuler.net/problem=16

use num_bigint::BigUint;
use num_traits::{pow, FromPrimitive};

pub fn solve(base: u32, exp: u32) -> u32 {
    let base: BigUint = FromPrimitive::from_u32(base).unwrap();
    pow(base, exp as usize)
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(2, 15), 26);
        assert_eq!(solve(2, 1000), 1366);
    }
}
