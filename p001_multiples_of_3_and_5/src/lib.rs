pub fn compute(n: usize) -> usize {
    (0..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(compute(1000), 233168);
    }
}
