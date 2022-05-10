struct Solution;

impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let (mut p, mut q, mut sum) = (1, 2, 3);
        while p < q {
            if sum == target {
                let mut arr = vec![0; q - p + 1];
                for i in p..=q {
                    arr[i - p] = i as i32;
                }
                result.push(arr);
                sum -= p as i32;
                p += 1;
                q += 1;
                sum += q as i32;
            } else if sum > target {
                sum -= p as i32;
                p += 1;
            } else {
                q += 1;
                sum += q as i32;
            }
        }

        let mut result_arr: Vec<Vec<i32>> = vec![vec![]; result.len()];
        for i in 0..result.len() {
            result_arr[i] = result[i].clone();
        }
        result_arr[..result.len()].clone_from_slice(&result[..]);
        result_arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_continuous_sequence(9),
            vec![vec![2, 3, 4], vec![4, 5]]
        );
        assert_eq!(
            Solution::find_continuous_sequence(15),
            vec![vec![1, 2, 3, 4, 5], vec![4, 5, 6], vec![7, 8]]
        );
    }
}
