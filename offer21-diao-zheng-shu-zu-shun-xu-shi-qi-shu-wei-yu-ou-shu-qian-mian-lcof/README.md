# 剑指 Offer 21. 调整数组顺序使奇数位于偶数前面

输入一个整数数组，实现一个函数来调整该数组中数字的顺序，使得所有奇数在数组的前半部分，所有偶数在数组的后半部分。



## 示例：

```
输入：nums = [1,2,3,4]
输出：[1,3,2,4]
注：[3,1,2,4] 也是正确的答案之一。
```

## 提示：

- 0 <= nums.length <= 50000
- 0 <= nums[i] <= 10000

## 题解
### Rust
```rust
impl Solution {
    pub fn exchange(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }
        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            if nums[i] % 2 == 1 {
                i += 1;
                continue;
            }
            if nums[j] % 2 == 0 {
                j -= 1;
                continue;
            }
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
        nums
    }
}
```
### Go
```go
package main

func exchange(nums []int) []int {
	i := 0
	j := len(nums) - 1
	for i < j {
		if nums[i]%2 == 1 {
			i++
			continue
		}
		if nums[j]%2 == 0 {
			j--
			continue
		}
		nums[i], nums[j] = nums[j], nums[i]
		i++
		j--
	}
	return nums
}

```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {number[]}
 */
var exchange = function(nums) {
    let i = 0
    let j = nums.length - 1
    while (i < j) {
        if (nums[i] % 2 === 1) {
            i++
            continue
        }
        if (nums[j] % 2 === 0) {
            j--
            continue
        }
        [nums[i], nums[j]] = [nums[j], nums[i]]
        i++
        j--
    }
    return nums
};
```

### Python
```python
class Solution:
    def exchange(self, nums: List[int]) -> List[int]:
        i = 0
        j = len(nums) -1
        while i < j:
            if nums[i] % 2 == 1:
                i += 1
                continue
            if nums[j] % 2 == 0:
                j -= 1
                continue
            tmp = nums[i]
            nums[i] = nums[j]
            nums[j] = tmp
            i += 1
            j -= 1
        return nums
```

### C++
```cpp
class Solution {
public:
    vector<int> exchange(vector<int>& nums) {
        int i = 0;
        int j = nums.size() - 1;
        while (i < j) {
            if (nums[i] % 2 == 1) {
                i++;
                continue;
            }
            if (nums[j] % 2 == 0) {
                j--;
                continue;
            }
            swap(nums[i], nums[j]);
            i++;
            j--;
        }
        return nums;
    }
};
```