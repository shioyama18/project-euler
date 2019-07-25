// Longest Collatz sequence
// https://projecteuler.net/problem=14
pub struct Collatz {
    curr: usize,
}

impl Collatz {
    pub fn new(n: usize) -> Self {
        Collatz { curr: n }
    }
}

impl Iterator for Collatz {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;

        match self.curr {
            1 => return None,
            x if x % 2 == 0 => self.curr /= 2,
            _ => self.curr = 3 * self.curr + 1,
        }

        Some(curr)
    }
}

pub fn solve(limit: usize) -> usize {
    let mut max_count = 0;
    let mut max_val = 0;

    for i in 2..=limit {
        let mut c = Collatz::new(i);
        let mut count = 1;

        while let Some(_) = c.next() {
            count += 1;
        }

        if count > max_count {
            max_count = count;
            max_val = i;
        }
    }

    max_val
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(1_000_000), 837799);
    }
}
