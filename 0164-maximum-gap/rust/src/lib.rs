struct Solution;

impl Solution {
    pub fn maximum_gap(nums:Vec<i32>) -> i32 {
        let mut arr = nums;
        arr.sort();
        let mut max = 0;
        for i in 1..arr.len() {
            let temp =  arr[i] - arr[i-1];
            println!("temp: {}", temp);
            if max <= temp{
                max = temp;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(Solution::maximum_gap(vec![1,10000000]), 9999999);
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
    }
}
