struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let (a, b) = (edges[0][0], edges[0][1]);
        let (c, d) = (edges[1][0], edges[1][1]);
        if a == c || a == d {
            return a;
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]),
            2
        );
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }
}
