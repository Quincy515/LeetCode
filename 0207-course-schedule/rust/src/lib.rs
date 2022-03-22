struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adjs = vec![vec![]; num_courses as usize];
        let mut indegrees = vec![0; num_courses as usize];
        for i in 0..prerequisites.len() {
            adjs[prerequisites[i][1] as usize].push(prerequisites[i][0]);
            indegrees[prerequisites[i][0] as usize] += 1;
        }
        let mut zero_in_degrees = vec![];
        for (i, _) in indegrees.iter().enumerate() {
            if indegrees[i] == 0 {
                zero_in_degrees.push(i);
            }
        }
        let mut zero_in_degrees_count = 0;
        while !zero_in_degrees.is_empty() {
            let coursei = zero_in_degrees.pop().unwrap();
            zero_in_degrees_count += 1;
            for coursej in &adjs[coursei] {
                indegrees[*coursej as usize] -= 1;
                if indegrees[*coursej as usize] == 0 {
                    zero_in_degrees.push(*coursej as usize);
                }
            }
        }

        zero_in_degrees_count == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_207() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
        assert!(Solution::can_finish(
            8,
            vec![
                vec![1, 0],
                vec![2, 6],
                vec![1, 7],
                vec![6, 4],
                vec![7, 0],
                vec![0, 5]
            ]
        ));
    }
}
