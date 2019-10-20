/// [Pentagon numbers](https://projecteuler.net/problem=44)
///
/// solve() returns min(|k-j|) such that sum and difference of two pentagonal numbers j and k are also pentagonal.
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p044::solve(), 5482660);
/// ```
pub fn solve() -> usize {
    let mut pentagons = vec![];
    let mut p = 1;

    for n in (4..).step_by(3) {
        pentagons.push(p);

        for &p_i in &pentagons {
            let p_k = p - p_i;
            let d = if p_i > p_k { p_i - p_k } else { p_k - p_i };

            if is_pentagonal(p_k) && is_pentagonal(d) {
                return d;
            }
        }

        p += n;
    }

    unreachable!()
}

fn is_pentagonal(x: usize) -> bool {
    let n = 1 + (1.0 + 24.0 * x as f64).sqrt() as usize / 6;
    let p_n = n * (3 * n - 1) / 2;
    p_n == x
}
