struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut left_products, mut right_products) = (vec![0; n], vec![0; n]);
        let mut product = 1;
        for i in 0..n {
            product *= nums[i];
            left_products[i] = product;
        }
        product = 1;
        for i in (0..=n - 1).rev() {
            product *= nums[i];
            right_products[i] = product;
        }
        let mut result = vec![0; n];
        for i in 0..n {
            result[i] = 1;
            if i >= 1 {
                result[i] *= left_products[i - 1];
            }
            if i + 1 < n {
                result[i] *= right_products[i + 1];
            }
        }
        result
    }
    pub fn product_except_self2(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        let mut left_product = 1;
        for i in 0..n {
            result[i] = left_product;
            left_product *= nums[i];
        }
        let mut right_product = 1;
        for i in (0..=n - 1).rev() {
            result[i] *= right_product;
            right_product *= nums[i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
