use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut exists = HashSet::with_capacity(nums.len());
        !nums.into_iter().all(|x| exists.insert(x))
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() || s.is_empty() || t.is_empty() {
            return false;
        }
        let mut s = s.into_bytes();
        s.sort_unstable();
        let mut t = t.into_bytes();
        t.sort_unstable();
        s == t
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
    #[test]
    pub fn is_anagram() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        )
    }
}
