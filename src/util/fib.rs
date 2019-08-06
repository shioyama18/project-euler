use num_traits::One;
use std::ops::Add;

#[derive(Default)]
pub struct Fibonacci<T> {
    curr: T,
    next: T,
}

impl<T: One> Fibonacci<T> {
    pub fn new() -> Self {
        Fibonacci {
            curr: One::one(),
            next: One::one(),
        }
    }
}

impl<T: Add<T, Output = T> + Clone> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let new_next = self.curr.clone() + self.next.clone();
        let new_curr = std::mem::replace(&mut self.next, new_next);
        let curr = std::mem::replace(&mut self.curr, new_curr);

        Some(curr)
    }
}
