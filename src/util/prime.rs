pub struct Prime {
    index: usize,
    primes: Vec<usize>,
}

impl Prime {
    pub fn new() -> Self {
        Prime {
            index: 0,
            primes: vec![2, 3],
        }
    }

    fn expand(&mut self) {
        let mut curr = self.primes.last().unwrap() + 2;

        loop {
            let mut remainder = 0;

            for prime in self.primes.iter() {
                remainder = curr % prime;
                if remainder == 0 || prime * prime > curr {
                    break;
                }
            }

            if remainder == 0 {
                curr += 2;
            } else {
                self.primes.push(curr);
                break;
            }
        }
    }
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index >= self.primes.len() {
            self.expand();
        }

        let n = self.primes[self.index];
        self.index += 1;
        Some(n)
    }
}
