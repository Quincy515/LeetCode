struct Solution;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let direction = [[0, 1], [1, 0], [0, -1], [-1, 0]]; //顺时针方向
        let (mut x, mut y, mut num, mut dir) = (r_start, c_start, 1, 0); //{x, y}为当前位置，num为当前查找的数字，dir为当前的方向
        let (mut left, mut right, mut upper, mut bottom) =
            (c_start - 1, c_start + 1, r_start - 1, r_start + 1); //四个方向的边界
        while num <= rows * cols {
            //{x, y} 位置在矩阵中
            if x >= 0 && x < rows && y >= 0 && y < cols {
                res.push(vec![x, y]);
                num += 1;
            }
            //向右到右边界
            if dir == 0 && y == right {
                dir += 1; //调转方向向下
                right += 1; //右边界右移
            }
            //向下到底边界
            else if dir == 1 && x == bottom {
                dir += 1;
                bottom += 1; //底边界下移
            }
            //向左到左边界
            else if dir == 2 && y == left {
                dir += 1;
                left -= 1; // 左边界左移
            }
            // 向上到上边界
            else if dir == 3 && x == upper {
                dir = 0;
                upper -= 1; //上边界上移
            }
            x += direction[dir][0]; // 下一个节点
            y += direction[dir][1];
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]
        );
        assert_eq!(
            Solution::spiral_matrix_iii(5, 6, 1, 4),
            vec![
                vec![1, 4],
                vec![1, 5],
                vec![2, 5],
                vec![2, 4],
                vec![2, 3],
                vec![1, 3],
                vec![0, 3],
                vec![0, 4],
                vec![0, 5],
                vec![3, 5],
                vec![3, 4],
                vec![3, 3],
                vec![3, 2],
                vec![2, 2],
                vec![1, 2],
                vec![0, 2],
                vec![4, 5],
                vec![4, 4],
                vec![4, 3],
                vec![4, 2],
                vec![4, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1, 1],
                vec![0, 1],
                vec![4, 0],
                vec![3, 0],
                vec![2, 0],
                vec![1, 0],
                vec![0, 0]
            ]
        );
    }
}
