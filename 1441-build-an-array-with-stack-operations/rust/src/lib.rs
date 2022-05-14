struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut index = 0;
        for i in 1..n + 1 {
            if target[index] == i {
                result.push("Push".to_string());
                index += 1;
                if index >= target.len() {
                    break;
                }
            } else {
                result.push("Push".to_string());
                result.push("Pop".to_string());
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec![
                "Push".to_owned(),
                "Push".to_owned(),
                "Pop".to_owned(),
                "Push".to_owned()
            ]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2, 3], 3),
            vec!["Push".to_owned(), "Push".to_owned(), "Push".to_owned()]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2], 3),
            vec!["Push".to_owned(), "Push".to_owned()]
        );
    }
}
