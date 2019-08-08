#[deny(missing_docs)]
/// [Number spiral diagonals](https://projecteuler.net/problem=28)
///
/// solve(n) returns sum of the numbers on the diagonals in a n by n spiral.
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p028::solve(5), 101);
/// assert_eq!(project_euler::solution::p028::solve(1001), 669171001);
/// ```
pub fn solve(n: usize) -> usize {
    std::iter::once(1)
        .chain(
            (2..n)
                .step_by(2)
                .map(|x| vec![x, x, x, x])
                .flat_map(|x| x.into_iter()),
        )
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .sum()
}
