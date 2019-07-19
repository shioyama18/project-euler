pub struct Prime {
    curr: usize,
    primes: Vec<usize>,
}

impl Prime {
    pub fn new() -> Self {
        Prime {
            curr: 2,
            primes: Vec::new(),
        }
    }

    fn is_prime(&self) -> bool {
        self.primes.iter().all(|x| self.curr % x != 0)
    }
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.is_prime() {
            self.curr += 1;
        }

        let n = self.curr;
        self.primes.push(n);
        Some(n)
    }
}
