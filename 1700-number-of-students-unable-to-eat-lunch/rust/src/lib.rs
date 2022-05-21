struct Solution;

impl Solution {
    pub fn count_students(mut students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 0);
        let mut count = [0, 0];
        for &i in &students {
            count[i as usize] += 1;
        }
        while i < students.len() && j < sandwiches.len() {
            if students[i] == sandwiches[j] {
                count[students[i] as usize] -= 1;
                i += 1;
                j += 1;
            } else {
                if count[0] == 0 || count[1] == 0 {
                    break;
                }
                students.push(students[i]);
                i += 1;
            }
        }
        (students.len() - i) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
