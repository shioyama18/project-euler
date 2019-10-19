/// [Sub-string divisibility](https://projecteuler.net/problem=43)
///
/// solve() returns sum of all 0 to 9 pandigital number that has the following property:
///
/// For pandigital number 1406357289
/// d2d3d4=406 is divisible by 2
/// d3d4d5=063 is divisible by 3
/// d4d5d6=635 is divisible by 5
/// d5d6d7=357 is divisible by 7
/// d6d7d8=572 is divisible by 11
/// d7d8d9=728 is divisible by 13
/// d8d9d10=289 is divisible by 17
///
/// # Example
///
/// ```
/// assert_eq!(project_euler::solution::p043::solve(), 16695334890);
/// ```
pub fn solve() -> usize {
    use permutohedron::Heap;

    let mut data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let heap = Heap::new(&mut data);

    heap.filter(|d| is_divisible(d))
        .map(|d| parse_digits(&d))
        .sum()
}

fn parse_digits(digits: &[usize]) -> usize {
    digits.iter().fold(0, |acc, x| acc * 10 + x)
}

fn is_divisible(digits: &[usize]) -> bool {
    digits[0] != 0
        && parse_digits(&digits[1..=3]) % 2 == 0
        && parse_digits(&digits[2..=4]) % 3 == 0
        && parse_digits(&digits[3..=5]) % 5 == 0
        && parse_digits(&digits[4..=6]) % 7 == 0
        && parse_digits(&digits[5..=7]) % 11 == 0
        && parse_digits(&digits[6..=8]) % 13 == 0
        && parse_digits(&digits[7..=9]) % 17 == 0
}
