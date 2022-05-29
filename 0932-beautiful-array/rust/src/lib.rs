struct Solution;


impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut result = vec![];
        if n == 1 {
            result.push(1);
            return result;
        }
        let odd_num = (n + 1) / 2;
        let even_num = n / 2;
        let (left_vec, right_vec) = (
            Self::beautiful_array(odd_num),
            Self::beautiful_array(even_num),
        );
        // 将左侧数组映射为奇数
        for v in left_vec.iter() {
            result.push(v * 2 - 1);
        }
        // 将右侧数组映射为偶数
        for v in right_vec.iter() {
            result.push(v * 2);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::beautiful_array(4), vec![2, 1, 4, 3]);
        assert_eq!(Solution::beautiful_array(5), vec![3, 1, 2, 5, 4]);
    }
}
