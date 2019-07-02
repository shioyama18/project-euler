fn main() {
    let solution = (0..1000)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum::<u32>();
    println!("Sum of all the multiples of 3 or 5 below 1000 is {}", solution);
}
