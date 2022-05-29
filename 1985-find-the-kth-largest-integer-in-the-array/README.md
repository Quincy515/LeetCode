# [1985.找出数组中的第 K 大整数](https://leetcode.cn/problems/find-the-kth-largest-integer-in-the-array/description/)

给你一个字符串数组 `nums` 和一个整数 `k` 。`nums` 中的每个字符串都表示一个不含前导零的整数。

返回 `nums` 中表示第 `k` 大整数的字符串。

**注意：**重复的数字在统计时会视为不同元素考虑。例如，如果 `nums` 是 `["1","2","2"]`，那么 `"2"` 是最大的整数，`"2"` 是第二大的整数，`"1"` 是第三大的整数。

 

**示例 1：**

```
输入：nums = ["3","6","7","10"], k = 4
输出："3"
解释：
nums 中的数字按非递减顺序排列为 ["3","6","7","10"]
其中第 4 大整数是 "3"
```

**示例 2：**

```
输入：nums = ["2","21","12","1"], k = 3
输出："2"
解释：
nums 中的数字按非递减顺序排列为 ["1","2","12","21"]
其中第 3 大整数是 "2"
```

**示例 3：**

```
输入：nums = ["0","0"], k = 2
输出："0"
解释：
nums 中的数字按非递减顺序排列为 ["0","0"]
其中第 2 大整数是 "0"
```

 

**提示：**

- `1 <= k <= nums.length <= 104`
- `1 <= nums[i].length <= 100`
- `nums[i]` 仅由数字组成
- `nums[i]` 不含任何前导零

------

[Discussion](https://leetcode.cn/problems/find-the-kth-largest-integer-in-the-array/comments/) | [Solution](https://leetcode.cn/problems/find-the-kth-largest-integer-in-the-array/solution/)

**思路**

自定义排序

**题解**

```rust
impl Solution {
    pub fn kth_largest_number(mut nums: Vec<String>, k: i32) -> String {
        // 自定义比较函数，在 a 对应的整数较大时返回 true，反之返回 false
        nums.sort_by(|a, b| {
			// 首先比较字符串长度
            if a.len() > b.len() {
                return std::cmp::Ordering::Greater;
            } else if a.len() < b.len() {
                return std::cmp::Ordering::Less;
            } else {
                // 长度相等时比较字符串字典序大小
                a.cmp(&b)
            }
        });

        nums[nums.len() - k as usize].to_string()
    }
}
```

[归并排序 Rust 代码模板](https://blog.csdn.net/custertian/article/details/125027099)

```rust
impl Solution {
    pub fn kth_largest_number(mut nums: Vec<String>, k: i32) -> String {
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
```
