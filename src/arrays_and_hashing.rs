use std::collections::{HashMap, HashSet};

mod pes;
mod sudoko;
pub use pes::*;
pub use sudoko::*;

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

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();
    for num in nums {
        map.entry(num).and_modify(|freq| *freq += 1).or_insert(1);
    }
    let mut xs = map.into_iter().collect::<Vec<_>>();
    xs.sort_unstable_by(|(_, x), (_, y)| y.cmp(x));
    xs.into_iter()
        .take(k as usize)
        .map(|(x, _)| x)
        .collect::<Vec<_>>()
}

pub fn encode(strs: &[String]) -> String {
    strs.join("#")
}
pub fn decode(string: String) -> Vec<String> {
    string.split('#').map(|x| x.to_string()).collect()
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let nums = nums.into_iter().collect::<HashSet<_>>();
    let mut max = 0;
    for num in nums.iter() {
        if !nums.contains(&(num - 1)) {
            let count = ((num + 1)..).take_while(|x| nums.contains(x)).count() + 1;
            if count > max {
                max = count;
            }
        }
    }
    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn contains_duplicate_test() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
    #[test]
    pub fn is_anagram_test() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false)
    }

    #[test]
    pub fn two_sum_test() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 3], 6), vec![0, 2]);
    }

    #[test]
    pub fn group_anagram() {
        assert_eq!(group_anagrams(vec!["".to_string(),]), vec![vec![""]]);
        assert_eq!(group_anagrams(vec!["a".to_string(),]), vec![vec!["a"]]);

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
            sort_it(group_anagrams(vec![
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
    #[test]
    pub fn top_k_fequent_element() {
        assert_eq!(
            top_k_frequent(vec![100, 100, 100, 200, 200, 300], 2),
            vec![100, 200]
        );
        assert_eq!(top_k_frequent(vec![100], 1), vec![100]);
        assert_eq!(top_k_frequent(vec![-1], -1), vec![-1]);
    }
    #[test]
    pub fn encode_decode() {
        let strings = vec![
            "i".to_string(),
            "love".to_string(),
            "problem".to_string(),
            "solving".to_string(),
        ];
        assert_eq!(decode(encode(&strings)), strings)
    }
    #[test]
    pub fn longest_consecutive_test() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
