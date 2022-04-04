# 面试题 16.24. 数对和
设计一个算法，找出数组中两数之和为指定值的所有整数对。一个数只能属于一个数对。

## 示例 1:

输入: nums = [5,6,5], target = 11
输出: [[5,6]]

## 示例 2:

输入: nums = [5,6,5,6], target = 11
输出: [[5,6],[5,6]]

## 提示：

nums.length <= 100000

## 题解
### Rust
```rust
impl Solution {
    pub fn pair_sums(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        nums.sort();
        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            if nums[i] + nums[j] == target {
                let mut result = vec![];
                result.push(nums[i]);
                result.push(nums[j]);
                results.push(result);
                i += 1;
                j -= 1;
            } else if nums[i] + nums[j] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        results
    }
}
```

### Go
```go
package main

import "sort"

func pairSums(nums []int, target int) [][]int {
	results := make([][]int, 0)
	if len(nums) == 0 {
		return results
	}
	sort.Ints(nums)
	i := 0
	j := len(nums) - 1
	for i < j {
		if nums[i]+nums[j] == target {
			result := make([]int, 0)
			result = append(result, nums[i])
			result = append(result, nums[j])
			results = append(results, result)
			i++
			j--
		} else if nums[i]+nums[j] < target {
			i++
		} else {
			j--
		}
	}
	return results
}
```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[][]}
 */
var pairSums = function (nums, target) {
  let results = [];
  if (nums.length === 0) {
    return results;
  }
  nums.sort((a, b) => a - b);
  let i = 0;
  let j = nums.length - 1;
  while (i < j) {
    if (nums[i] + nums[j] === target) {
      results.push([nums[i], nums[j]]);
      i++;
      j--;
    } else if (nums[i] + nums[j] < target) {
      i++;
    } else {
      j--;
    }
  }
  return results;
}
```

### Python
```python
class Solution:
    def pairSums(self, nums: List[int], target: int) -> List[List[int]]:
        result = []
        if len(nums) == 0:
            return result
        nums.sort()
        i = 0
        j = len(nums) - 1
        while i < j:
            if nums[i] + nums[j] == target:
                result.append([nums[i], nums[j]])
                i += 1
                j -= 1
            elif nums[i] + nums[j] < target:
                i += 1
            else:
                j -= 1
        return result
```

### C++
```c++
class Solution {
public:
    vector<vector<int>> pairSums(vector<int> &nums, int target) {
        vector<vector<int>> results;
        if (nums.size() == 0) return result;
        std::sort(nums.begin(), nums.end());
        int i = 0;
        int j = nums.size() - 1;
        while (i < j) {
            if (nums[i] + nums[j] == target) {
                results.push_back({nums[i], nums[j]});
                i++;
                j--;
            } else if (nums[i] + nums[j] < target) {
                i++;
            } else {
                j--;
            }
        }
        return results;
    }
};
```