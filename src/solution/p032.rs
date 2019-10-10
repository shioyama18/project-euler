/// [Panditital products](https://projecteuler.net/problem=32)
///
/// solve() returns sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p032::solve(), 45228);
/// ```
pub fn solve() -> usize {
    let mut pandigitals = std::collections::HashSet::new();

    for i in 2..60 {
        let start = if i < 10 { 1234 } else { 123 };
        for j in start..10000 / i {
            let k = i * j;
            let n = i.to_string() + &j.to_string() + &k.to_string();
            if is_pandigital(n) {
                pandigitals.insert(k);
            }
        }
    }

    pandigitals.into_iter().sum()
}

fn is_pandigital(n: String) -> bool {
    if n.len() != 9 {
        return false;
    }
    let mut n = n.chars().collect::<Vec<char>>();
    n.sort();
    n.into_iter().collect::<String>() == "123456789"
}
