// Highly divisible triangular number
// https://projecteuler.net/problem=12

pub fn solve(n: usize) -> usize {
    let mut i = 1;
    let mut c = 2;

    loop {
        if num_factors(i) > n {
            break;
        }
        i += c;
        c += 1;
    }

    i
}

fn num_factors(n: usize) -> usize {
    (1..)
        .take_while(|i| i * i <= n)
        .filter(|i| n % i == 0)
        .count()
        * 2
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
