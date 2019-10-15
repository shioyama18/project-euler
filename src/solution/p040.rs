/// [Champernowne's constant](https://projecteuler.net/problem=40)
///
/// An irrational decimal fraction is created by concatenating the positive integers:
/// i.e. 0.1234567891011121314...
///
/// With d1 representing 1st fractional digit and d10 representing 10th fractional digit,
/// solve() returns the following expression:
/// d1 * d10 * d100 * d1000 * d10000 * d100000 * d1000000
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p040::solve(),210);
/// ```
pub fn solve() -> usize {
    let digits = IrrationalDecimal::new();
    let digits = std::iter::repeat(0)
        .take(1)
        .chain(digits.into_iter().take(1_000_000))
        .collect::<Vec<_>>();

    digits[1]
        * digits[10]
        * digits[100]
        * digits[1000]
        * digits[10000]
        * digits[100000]
        * digits[1000000]
}

struct IrrationalDecimal {
    index: usize,
    current: usize,
    digits: Vec<usize>,
}

impl IrrationalDecimal {
    fn new() -> Self {
        IrrationalDecimal {
            index: 0,
            current: 1,
            digits: Vec::new(),
        }
    }

    fn expand(&mut self) {
        let current = self
            .current
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        current
            .into_iter()
            .for_each(|x| self.digits.push(x as usize));
        self.current += 1;
    }
}

impl Iterator for IrrationalDecimal {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index >= self.digits.len() {
            self.expand();
        }

        let d = self.digits[self.index];
        self.index += 1;
        Some(d)
    }
}
