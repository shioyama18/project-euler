// Factorial digit sum
// https://projecteuler.net/problem=20

use num_bigint::BigUint;
use num_traits::FromPrimitive;

pub fn solve(n: u32) -> u32 {
    num_iter::range::<BigUint>(
        FromPrimitive::from_u32(2).unwrap(),
        FromPrimitive::from_u32(n + 1).unwrap(),
    )
    .product::<BigUint>()
    .to_string()
    .chars()
    .filter_map(|i| i.to_digit(10))
    .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(10), 27);
        assert_eq!(solve(100), 648);
    }
}
