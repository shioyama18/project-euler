// Sum square difference
// https://projecteuler.net/problem=6

pub fn solve(n: usize) -> usize {
    let sum_squares = (1..=n).map(|x| x * x).sum::<usize>();
    let sum = n * (n + 1) / 2;
    let square_sum = sum * sum;
    square_sum - sum_squares
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(10), 2640);
        assert_eq!(solve(100), 25164150);
    }
}
