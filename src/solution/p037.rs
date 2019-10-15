/// [Truncatable primes](https://projecteuler.net/problem=37)
///
/// solve() returns sum of 11 primes that are truncatable from both side
/// i.e. 3797 -> 797 -> 97 -> 7 and 3797 -> 379 -> 37 -> 3 are all primes
///
/// # Example
///
/// ```ignore
/// assert_eq!(project_euler::solution::p037::solve(), 748317);
/// ```
pub fn solve() -> usize {
    use crate::util::prime::Prime;

    let mut count = 0;
    let mut sum = 0;
    let prime_iter = Prime::new().skip_while(|&p| p <= 7);
    let mut primes = vec![2, 3, 5, 7];

    for p in prime_iter {
        if count == 11 {
            break;
        }

        primes.push(p);

        if is_truncatable_prime(p, &primes) {
            sum += p;
            count += 1;
        }
    }

    sum
}

fn is_truncatable_prime(p: usize, primes: &[usize]) -> bool {
    let p = p
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as usize)
        .collect::<Vec<_>>();

    let digits = p.len();
    let mut truncated = vec![];

    for i in 1..digits {
        truncated.push(slice_to_num(&p[i..]));
        truncated.push(slice_to_num(&p[..i]));
    }

    truncated.iter().all(|t| primes.contains(t))
}

fn slice_to_num(p: &[usize]) -> usize {
    p.iter().fold(0, |acc, x| acc * 10 + x)
}
