/// [Circular primes](https://projecteuler.net/problem=35)
///
/// solve(n) returns number of circular primes below n.
/// Circular primes are numbers such that rotation of the digits are all primes.
/// i.e. 197 -> 197, 971, 719 are all primes
///
/// # Example
///
/// ```ignore
/// assert_eq!(project_euler::solution::p035::solve(1_000_000), 55);
/// ```
pub fn solve(n: usize) -> usize {
    if n <= 2 {
        return 0;
    }

    let primes = crate::util::prime::Prime::upto(n);
    let mut circular = vec![];

    for &p in &primes {
        if is_circular_prime(p, &primes) {
            circular.push(p);
        }
    }

    circular.len()
}

fn is_circular_prime(p: usize, primes: &[usize]) -> bool {
    let rotation = generate_rotation(p);

    rotation.iter().all(|n| primes.contains(n))
}

fn generate_rotation(n: usize) -> Vec<usize> {
    let mut n = n
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    let mut rotations = vec![slice_to_num(&n)];

    for _ in 1..n.len() {
        n.rotate_left(1);
        rotations.push(slice_to_num(&n));
    }

    rotations
}

fn slice_to_num(v: &[u32]) -> usize {
    v.iter().fold(0, |acc, x| acc * 10 + x) as usize
}
