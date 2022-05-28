struct Solution;
impl Solution {
    pub fn kth_largest_number(mut nums: Vec<String>, k: i32) -> String {
        nums.sort_by(|a, b| {
            if a.len() > b.len() {
                return std::cmp::Ordering::Greater;
            } else if a.len() < b.len() {
                return std::cmp::Ordering::Less;
            } else {
                a.cmp(&b)
            }
        });

        nums[nums.len() - k as usize].to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::kth_largest_number(
                vec![
                    "3".to_string(),
                    "6".to_string(),
                    "7".to_string(),
                    "10".to_string(),
                ],
                4,
            ),
            "3".to_string()
        );
        assert_eq!(
            Solution::kth_largest_number(
                vec![
                    "2".to_string(),
                    "21".to_string(),
                    "12".to_string(),
                    "1".to_string(),
                ],
                3,
            ),
            "2".to_string()
        );
        assert_eq!(
            Solution::kth_largest_number(vec!["0".to_string(), "0".to_string()], 2),
            "0".to_string()
        );
    }
}
