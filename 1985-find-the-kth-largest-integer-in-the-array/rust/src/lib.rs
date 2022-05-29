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

    pub fn kth_largest_number_merge_sort(mut nums: Vec<String>, k: i32) -> String {
        let n = nums.len() - 1;
        merge_sort(&mut nums, 0, n);
        nums[nums.len() - k as usize].to_string()
    }
}

fn a_big_than_b(a: &String, b: &String) -> bool {
    if a.len() != b.len() {
        return a.len() > b.len();
    }
    a > b
}

fn merge_sort(nums: &mut Vec<String>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let mid = left + (right - left) / 2;
    merge_sort(nums, left, mid);
    merge_sort(nums, mid + 1, right);
    merge(nums, left, mid, right);
}

fn merge(nums: &mut Vec<String>, left: usize, mid: usize, right: usize) {
    let (mut i, mut j, mut k) = (left, mid + 1, left);
    let mut temp = vec![];
    while k <= right {
        if i > mid {
            temp.push(nums[j].to_string());
            j += 1;
            k += 1;
        } else if j > right {
            temp.push(nums[i].to_string());
            i += 1;
            k += 1;
        } else {
            if a_big_than_b(&nums[i], &nums[j]) {
                temp.push(nums[j].to_string());
                j += 1;
                k += 1;
            } else {
                temp.push(nums[i].to_string());
                i += 1;
                k += 1;
            }
        }
    }

    for i in 0..=(right - left) {
        nums[left + i] = temp[i].to_string();
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
