fn main() {
    let mut largest_parlindrom = 0;

    for i in 100..1000 {
        for j in i..1000 {
            let k = i * j;
            let n = k.to_string();
            let n_rev = k.to_string().chars().rev().collect::<String>();

            if n == n_rev && k > largest_parlindrom {
                largest_parlindrom = k;
            }
        }
    }

    println!("{}", largest_parlindrom);
}
