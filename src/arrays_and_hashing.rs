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

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let state = ProductExeceptSelfState::from(nums);
        state.get()
    }

    pub fn is_valid_sodoku(board: Vec<Vec<char>>) -> bool {
        let mut set = HashSet::with_capacity(81);
        !board
            .into_iter()
            .flatten()
            .enumerate()
            .filter(|x| x.1 != '.')
            .map(|(index, letter)| SodokuClass::from(letter, index + 1))
            .any(|x| !set.insert(x))
    }
}

#[derive(Debug, Eq, Hash)]
struct SodokuClass {
    row: usize,
    column: usize,
    area: usize,
    char: char,
}

impl PartialEq for SodokuClass {
    fn eq(&self, other: &Self) -> bool {
        self.char == other.char
            && (self.column == other.column || self.row == other.row || self.area == other.area)
    }
}

impl SodokuClass {
    fn from(char: char, index: usize) -> Self {
        let column = index % 9;
        let row = (index - column) / 9;
        let area = match (row, column) {
            (0..=2, 0..=2) => 1,
            (3..=5, 3..=5) => 2,
            (6..=9, 6..=8) => 3,
            (0..=2, 3..=5) => 4,
            (0..=2, 6..=8) => 5,
            (3..=5, 0..=2) => 6,
            (3..=5, 6..=8) => 7,
            (6..=9, 0..=2) => 8,
            (6..=9, 3..=5) => 9,
            _ => unreachable!(),
        };

        Self {
            row,
            column,
            area,
            char,
        }
    }
}

enum ProductExeceptSelfState {
    NoZero(Vec<i32>, i32),
    SingleZero(usize, usize, i32),
    SomeZeros(usize),
}

impl ProductExeceptSelfState {
    fn from(nums: Vec<i32>) -> Self {
        enum Assume {
            NoZero,
            SingleZero(usize, usize),
            SomeZeros(usize),
        }
        let mut all_product = 1;
        let mut non_zero_product = 1;
        let mut state = Assume::NoZero;
        for (index, num) in nums.iter().enumerate() {
            if num == &0 {
                match state {
                    Assume::SomeZeros(_) => (),
                    Assume::SingleZero(_, _) => state = Assume::SomeZeros(nums.len()),
                    Assume::NoZero => state = Assume::SingleZero(index, nums.len()),
                }
            } else {
                non_zero_product *= num;
            }
            all_product *= num;
        }
        match state {
            Assume::NoZero => ProductExeceptSelfState::NoZero(nums, all_product),
            Assume::SingleZero(index, len) => {
                ProductExeceptSelfState::SingleZero(index, len, non_zero_product)
            }
            Assume::SomeZeros(len) => ProductExeceptSelfState::SomeZeros(len),
        }
    }
    fn get(self) -> Vec<i32> {
        match self {
            ProductExeceptSelfState::SomeZeros(len) => vec![0; len],
            ProductExeceptSelfState::SingleZero(index, len, non_zero_product) => {
                let mut arr = vec![0; len];
                arr[index] = non_zero_product;
                arr
            }
            ProductExeceptSelfState::NoZero(nums, all_product) => {
                nums.into_iter().map(|x| all_product / x).collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{arrays_and_hashing::SodokuClass, Solution};

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
    #[test]
    pub fn top_k_fequent_element() {
        assert_eq!(
            Solution::top_k_frequent(vec![100, 100, 100, 200, 200, 300], 2),
            vec![100, 200]
        );
        assert_eq!(Solution::top_k_frequent(vec![100], 1), vec![100]);
        assert_eq!(Solution::top_k_frequent(vec![-1], -1), vec![-1]);
    }
    #[test]
    pub fn encode_decode() {
        let strings = vec![
            "i".to_string(),
            "love".to_string(),
            "problem".to_string(),
            "solving".to_string(),
        ];
        assert_eq!(Solution::decode(Solution::encode(&strings)), strings)
    }
    #[test]
    pub fn product_except_self() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );

        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );

        assert_eq!(Solution::product_except_self(vec![0, 0]), vec![0, 0]);
        assert_eq!(Solution::product_except_self(vec![0, 4, 0]), vec![0, 0, 0]);
    }
    #[test]
    pub fn is_valid_sodoku() {
        assert_eq!(SodokuClass::from('a', 2), SodokuClass::from('a', 11));
        assert_eq!(SodokuClass::from('a', 2), SodokuClass::from('a', 7));
        assert_eq!(SodokuClass::from('a', 2), SodokuClass::from('a', 1));

        assert!(Solution::is_valid_sodoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]));

        assert!(!Solution::is_valid_sodoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));

        assert!(!Solution::is_valid_sodoku(vec![
            vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
            vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.']
        ]));
    }
}
