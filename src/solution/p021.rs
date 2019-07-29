// Amicable numbers
// https://projecteuler.net/problem=21

// TODO: optimize

pub fn solve(n: usize) -> usize {
    (2..n)
        .filter(|&i| {
            let x = sum_divisor(i);
            i == sum_divisor(x) && i != x
        })
        .sum()
}

fn sum_divisor(n: usize) -> usize {
    (1..n).filter(|x| n % x == 0).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(10000), 31626);
    }
}
