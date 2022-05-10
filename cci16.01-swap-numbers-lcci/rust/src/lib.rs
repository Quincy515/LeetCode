struct Solution;

impl Solution {
    pub fn swap_numbers(mut numbers: Vec<i32>) -> Vec<i32> {
        if numbers[0] == numbers[1] {
            return numbers;
        }
        numbers.swap(0, 1);
        numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::swap_numbers(vec![1, 2]), vec![2, 1]);
    }
}
