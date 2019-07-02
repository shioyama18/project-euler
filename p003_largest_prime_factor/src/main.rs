fn is_prime(n: u64, primes: &mut Vec<u64>) -> bool {
    if primes.iter().all(|x| n % x != 0) {
        primes.push(n);
        return true;
    }
    return false;
}

fn main() {
    let mut primes = Vec::new();
    let target = 600_851_475_143u64;

    let largest_prime = (2..).take_while(|x| x * x < target)
        .filter(|&x| is_prime(x, &mut primes) && target % x == 0)
        .max().expect("Largest prime number below 600851475143");;

    println!("{}", largest_prime);
}
