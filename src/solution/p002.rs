// Even Fibonacci numbers
// https://projecteuler.net/problem=2

use crate::util::fib::Fibonacci;

pub fn solve(max: usize) -> usize {
    let fib = Fibonacci::new();
    fib.take_while(|&x| x < max).filter(|x| x % 2 == 0).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(4_000_000), 4_613_732);
    }
}
