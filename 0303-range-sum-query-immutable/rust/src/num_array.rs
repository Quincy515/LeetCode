// sum[i] 存储前 i 个元素和 sum[0]=0
// sum[i] 存储 nums[0...i-1] 的和
struct NumArray {
    sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum = vec![0; nums.len() + 1];
        for i in 1..sum.len() {
            sum[i] = sum[i - 1] + nums[i - 1];
        }
        Self { sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum[right as usize + 1] - self.sum[left as usize]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let segment_tree = NumArray::new(nums);
        println!("{:?}", segment_tree.sum_range(0, 2));
        println!("{:?}", segment_tree.sum_range(2, 5));
        println!("{:?}", segment_tree.sum_range(0, 5));
    }
}
