use std::mem::swap;

const MAX: u32 = 4_000_000;

fn main() {
    let mut n1 = 1;
    let mut n2 = 1;
    let mut sum = 0;

    while n1 < MAX {
        n1 += n2;
        swap(&mut n1, &mut n2);
        if n1 % 2 == 0 {
            sum += n1;
        }
    }

    println!("{}", sum);
}
