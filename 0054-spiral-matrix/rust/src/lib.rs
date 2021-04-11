struct Solution;
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        // 定义上下左右四个边界并初始化
        let (mut top, mut bottom, mut left, mut right) = (0, matrix.len(), 0, matrix[0].len());
        // 接着不断循环以下操作
        loop {
            // 1. 先从左到右遍历上边界，把元素加到结果列表
            for i in left..right {
                res.push(matrix[top][i]);
            }
            top += 1;
            if top == bottom {
                break;
            }
            // 2. 然后从上向下遍历右边界，把元素加到结果列表
            for i in top..bottom {
                res.push(matrix[i][right - 1]);
            }
            right -= 1;
            if right == left {
                break;
            }
            // 3. 然后从右向左遍历下边界，把元素加到结果列表
            for i in (left..right).rev() {
                res.push(matrix[bottom - 1][i]);
            }
            bottom -= 1;
            if bottom == top {
                break;
            }
            // 4. 然后从下向上遍历左边界，把元素加到结果列表
            for i in (top..bottom).rev() {
                res.push(matrix[i][left]);
            }
            left += 1;
            if left == right {
                break;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(matrix)
        );
    }

    #[test]
    fn it_works2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            Solution::spiral_order(matrix)
        );
    }

    #[test]
    fn it_works3() {
        let matrix = vec![vec![3], vec![2]];
        assert_eq!(vec![3, 2], Solution::spiral_order(matrix));
    }
}
