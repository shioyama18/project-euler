// Lattice paths
// https://projecteuler.net/problem=15

pub fn solve(n: usize) -> usize {
    // Code below overflows
    // (n + 1..=n + n).product::<usize>() / (2..=n).product::<usize>()

    let numerator = n + 1..=n + n;
    let mut denominator = (2..=n).peekable();
    let mut answer = 1;

    for n in numerator {
        if n * answer < std::usize::MAX {
            answer *= n;
        }

        loop {
            match denominator.peek() {
                Some(x) if answer % x == 0 => answer /= denominator.next().unwrap(),
                _ => break,
            }
        }
    }

    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(2), 6);
        assert_eq!(solve(20), 137846528820);
    }
}
