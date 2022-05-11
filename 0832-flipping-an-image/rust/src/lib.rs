struct Solution;
impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (row, col) = (image.len(), image.first().map_or(0, Vec::len));
        for i in 0..row {
            for j in 0..col / 2 {
                image[i].swap(j, col - 1 - j);
            }
            for j in 0..col {
                image[i][j] ^= 1;
            }
        }

        image
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
        );
        assert_eq!(
            Solution::flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ]),
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ]
        );
    }
}
