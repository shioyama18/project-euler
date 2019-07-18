pub struct Fibonacci {
    curr: usize,
    next: usize,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { curr: 1, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

pub fn compute(max: usize) -> usize {
    let fib = Fibonacci::new();
    fib.take_while(|&x| x < max).filter(|x| x % 2 == 0).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(compute(4_000_000), 4_613_732);
    }
}
