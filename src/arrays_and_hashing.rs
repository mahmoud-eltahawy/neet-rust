use std::collections::{HashMap, HashSet};

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

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::with_capacity(nums.len());
        for (index, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(x) = map.get(&diff) {
                return vec![*x, index as i32];
            }
            map.insert(num, index as i32);
        }
        unreachable!()
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::<Vec<char>, Vec<String>>::new();

        for word in strs {
            let mut sorted_word = word.chars().collect::<Vec<_>>();
            sorted_word.sort_unstable();
            match map.get_mut(&sorted_word) {
                Some(arr) => arr.push(word),
                None => {
                    map.insert(sorted_word, vec![word]);
                }
            }
        }

        map.values().cloned().collect()
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

    #[test]
    pub fn two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 3], 6), vec![0, 2]);
    }

    #[test]
    pub fn group_anagram() {
        assert_eq!(
            Solution::group_anagrams(vec!["".to_string(),]),
            vec![vec![""]]
        );
        assert_eq!(
            Solution::group_anagrams(vec!["a".to_string(),]),
            vec![vec!["a"]]
        );

        let sort_it = |arr: Vec<Vec<_>>| {
            let mut arr = arr
                .into_iter()
                .map(|xs| {
                    let mut xs = xs;
                    xs.sort_unstable();
                    xs
                })
                .collect::<Vec<_>>();
            arr.sort_unstable();
            arr
        };

        assert_eq!(
            sort_it(Solution::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ])),
            sort_it(vec![
                vec!["bat".to_string()],
                vec!["nat".to_string(), "tan".to_string()],
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]
            ])
        );
    }
}
