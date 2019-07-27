// Number letter counts
// https://projecteuler.net/problem=17

const ONE: [&str; 19] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TY: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

pub fn solve(n: usize) -> usize {
    let mut words = Vec::new();

    for i in 1..=n {
        let mut ns = String::new();
        decompose(i, &mut ns);
        words.push(ns.len());
    }

    words.into_iter().sum()
}

fn decompose(x: usize, dict: &mut String) {
    match x {
        0 => return,
        x if x < 20 => dict.push_str(ONE[x - 1]),
        x if x >= 20 && x < 100 => {
            dict.push_str(TY[first_digit(x) - 2]);
            decompose(x - first_digit(x) * 10, dict)
        }
        x if x < 1000 && x % 100 == 0 => {
            dict.push_str(ONE[first_digit(x) - 1]);
            dict.push_str("hundred");
        }
        x if x > 100 && x <= 999 => {
            dict.push_str(ONE[first_digit(x) - 1]);
            dict.push_str("hundredand");
            decompose(x - first_digit(x) * 100, dict);
        }
        x if x == 1000 => dict.push_str("onethousand"),
        _ => panic!("Number above 1000 is not implemented"),
    }
}

fn first_digit(x: usize) -> usize {
    let first = x.to_string().chars().next().unwrap();
    first.to_digit(10).unwrap() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(5), 19);
        assert_eq!(solve(1000), 21124);
    }
}
