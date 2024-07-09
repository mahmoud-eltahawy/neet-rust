pub struct Solution;

impl Solution {
    pub fn increment(num: i32) -> i32 {
        num + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn increment() {
        assert_eq!(Solution::increment(1), 2)
    }
}
