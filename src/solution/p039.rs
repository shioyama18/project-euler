/// [Integer right triangles](https://projecteuler.net/problem=39)
///
/// solve(n) returns the perimeter greater than 12 and less than or equal to n that produces the most number of right triangles.
/// i.e. For n = 120, there are three right triangles {20,48,52}, {24,45,51}, {30,40,50}
///
/// # Example
///
/// ```ignore
/// assert_eq!(project_euler::solution::p039::solve(1000), 840);
/// ```
pub fn solve(n: usize) -> usize {
    let mut p = 12;
    let mut max = 1;

    for i in 12..=n {
        let num_triangles = generate_right_triangles(i);
        if num_triangles > max {
            max = num_triangles;
            p = i;
        }
    }

    p
}

fn generate_right_triangles(p: usize) -> usize {
    let mut count = 0;

    for a in 1..p {
        for b in a..p - a {
            let c = p - (a + b);
            if a * a + b * b == c * c {
                count += 1;
            }
        }
    }

    count
}
