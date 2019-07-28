// Counting Sundays
// https://projecteuler.net/problem=19

const LEAP: [usize; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const NON_LEAP: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

enum Weekday {
    Sunday,
    Monday,
}

pub fn solve() -> usize {
    let four_years = LEAP.iter().chain(NON_LEAP.iter().cycle().take(36));

    let x = NON_LEAP.iter().cycle().take(48).chain(four_years.cycle());

    let x = x.scan(Weekday::Monday as usize, |state, &x| {
        *state = (*state + x) % 7;
        Some(*state)
    });

    x.take(1212)
        .skip(12)
        .filter(|&x| x == Weekday::Sunday as usize)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(), 171);
    }
}
