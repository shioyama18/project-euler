/// [Goldbach's other conjecture](https://projecteuler.net/problem=46)
///
/// solve() returns the smallest odd composite that cannot be written as the sum of a prime and twice a square
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p046::solve(), 5777);
/// ```
pub fn solve() -> usize {
    let primes = crate::util::prime::Prime::upto(6000);

    (3..)
        .step_by(2)
        .filter(|n| !primes.contains(n))
        .skip_while(|&n| is_goldbach(&primes, n))
        .next()
        .unwrap()
}

fn is_goldbach(primes: &[usize], n: usize) -> bool {
    let limit = (n as f64 / 2.0).sqrt() as usize + 1;
    for x in 1..limit {
        let twice_sq = x * x * 2;

        if twice_sq > n {
            return false;
        }

        if primes.contains(&(n - twice_sq)) {
            return true;
        }
    }
    false
}
