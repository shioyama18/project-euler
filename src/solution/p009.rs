// Special Pythagorean triplet
// https://projecteuler.net/problem=9

pub fn solve(total: usize) -> usize {
    let limit = (total as f64).sqrt() as usize;

    for i in 2..=limit {
        for j in 1..i {
            let a = i * i - j * j;
            let b = 2 * i * j;
            let c = i * i + j * j;

            if a + b + c == total {
                return a * b * c;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(12), 60);
        assert_eq!(solve(1000), 31875000);
    }
}
