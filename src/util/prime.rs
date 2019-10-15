#[derive(Default)]
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

    pub fn upto(n: usize) -> Vec<usize> {
        let primes = Self::new();
        primes.take_while(|&p| p < n).collect()
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

    pub fn prime_factors(mut n: usize) -> Vec<usize> {
        let p = Prime::new();
        let mut prime_factors = Vec::new();

        for x in p {
            if x * x > n {
                prime_factors.push(n);
                break;
            }

            while n % x == 0 {
                n /= x;
                prime_factors.push(x);
            }

            if n == 1 {
                break;
            }
        }

        prime_factors
    }

    pub fn sum_divisor(n: usize) -> usize {
        if n == 1 {
            return 1;
        }

        use std::collections::HashMap;

        let dict = {
            let pf = Self::prime_factors(n);
            let mut dict = HashMap::new();
            pf.into_iter()
                .for_each(|i| *dict.entry(i).or_insert(0) += 1);
            dict
        };

        dict.into_iter()
            .map(|(base, exp)| (base.pow(exp + 1) - 1) / (base - 1))
            .product::<usize>()
    }

    pub fn sum_proper_divisor(n: usize) -> usize {
        Self::sum_divisor(n) - n
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
