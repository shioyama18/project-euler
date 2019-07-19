// Largest palindrome product
// https://projecteuler.net/problem=4

pub fn solve(min: usize, max: usize) -> usize {
    let mut largest_parlindrom = 0;

    for i in min..=max {
        for j in i..=max {
            let k = i * j;
            let n = k.to_string();
            let n_rev = k.to_string().chars().rev().collect::<String>();

            if n == n_rev && k > largest_parlindrom {
                largest_parlindrom = k;
            }
        }
    }

    largest_parlindrom
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(100, 999), 906609);
    }
}
