use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<&str> = version1.split('.').collect();
        let v2: Vec<&str> = version2.split('.').collect();
        let (n1, n2) = (v1.len(), v2.len());
        if n1 >= n2 {
            for i in 0..n2 {
                if v1[i].parse::<i32>().unwrap() > v2[i].parse::<i32>().unwrap() {
                    return 1;
                } else if v1[i].parse::<i32>().unwrap() < v2[i].parse::<i32>().unwrap() {
                    return -1;
                }
            }
            for i in n2..n1 {
                if v1[i].parse::<i32>().unwrap() > 0 {
                    return 1;
                }
            }
        } else {
            for i in 0..n1 {
                if v1[i].parse::<i32>().unwrap() > v2[i].parse::<i32>().unwrap() {
                    return 1;
                } else if v1[i].parse::<i32>().unwrap() < v2[i].parse::<i32>().unwrap() {
                    return -1;
                }
            }
            for i in n1..n2 {
                if v2[i].parse::<i32>().unwrap() > 0 {
                    return -1;
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::compare_version("1.01".to_string(), "1.001".to_string()),
            0
        );
        assert_eq!(
            Solution::compare_version("1.0".to_string(), "1.0.0".to_string()),
            0
        );
        assert_eq!(
            Solution::compare_version("0.1".to_string(), "1.1".to_string()),
            -1
        );
    }
}
