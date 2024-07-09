use std::ops::{Add, Sub};

pub struct PrefixSum<T: Add<Output = T> + Sub<Output = T> + Clone> {
    list: Vec<T>,
}

impl<T: Add<Output = T> + Sub<Output = T> + Clone> PrefixSum<T> {
    pub fn from(arr: Vec<T>) -> Self {
        let mut list = Vec::<T>::with_capacity(arr.len());
        list.push(arr[0].clone());
        for i in 1..arr.len() {
            list.push(arr[i].clone() + list[i - 1].clone());
        }

        Self { list }
    }

    pub fn from_to(&self, begin: usize, end: usize) -> T {
        if begin == 0 {
            return self.list[end].clone();
        }
        self.list[end].clone() - self.list[begin - 1].clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn prefix_sum_test() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let ps = PrefixSum::from(list);

        assert_eq!(ps.from_to(1, 3), 9);
        assert_eq!(ps.from_to(2, 5), 18);
        assert_eq!(ps.from_to(7, 9), 27);

        let PrefixSum { list } = ps;
        assert_eq!(list, vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
    }
}
