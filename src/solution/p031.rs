const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

#[deny(missing_docs)]
/// [Coin sums](https://projecteuler.net/problem=31)
///
/// solve(n) returns number of ways to make up n pences.
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p031::solve(200), 73682);
/// ```
pub fn solve(n: usize) -> usize {
    let mut num_ways = vec![0; n];
    num_ways.insert(0, 1);
    let len = num_ways.len();

    for &c in COINS.iter() {
        if len < c {
            break;
        }

        for i in 0..len - c {
            num_ways[i + c] += num_ways[i];
        }
    }

    num_ways.pop().unwrap()
}
