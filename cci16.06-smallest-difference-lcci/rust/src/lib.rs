struct Solution;

impl Solution {
    pub fn smallest_difference(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
        a.sort();
        b.sort();
        let (n, m) = (a.len(), b.len());
        let mut min_ret = i64::MAX;
        let (mut i, mut j) = (0, 0);
        while i < n && j < m {
            if a[i] >= b[j] {
                min_ret = i64::abs(min_ret.min(a[i] as i64 - b[j] as i64));
                j += 1;
            } else {
                min_ret = i64::abs(min_ret.min(b[j] as i64 - a[i] as i64));
                i += 1;
            }
        }
        min_ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::smallest_difference(vec![-2147483648, 1], vec![2147483647, 0]),
            1
        );
        assert_eq!(
            Solution::smallest_difference(vec![1, 3, 15, 11, 2], vec![23, 127, 235, 19, 8]),
            3
        );
    }
}
