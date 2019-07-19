// Multiples of 3 and 5
// https://projecteuler.net/problem=1

pub fn solve(n: usize) -> usize {
    (0..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(1000), 233168);
    }
}
