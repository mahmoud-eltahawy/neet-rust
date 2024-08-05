pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let state = ProductExeceptSelfState::from(nums);
    state.get()
}

pub enum ProductExeceptSelfState {
    NoZero(Vec<i32>, i32),
    SingleZero(usize, usize, i32),
    SomeZeros(usize),
}

impl ProductExeceptSelfState {
    pub fn from(nums: Vec<i32>) -> Self {
        pub enum Assume {
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
    pub fn get(self) -> Vec<i32> {
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
    use super::*;
    #[test]
    pub fn product_except_self_test() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);

        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );

        assert_eq!(product_except_self(vec![0, 0]), vec![0, 0]);
        assert_eq!(product_except_self(vec![0, 4, 0]), vec![0, 0, 0]);
    }
}
