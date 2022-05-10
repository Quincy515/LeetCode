# 面试题 17.04. 消失的数字

数组nums包含从0到n的所有整数，但其中缺了一个。请编写代码找出那个缺失的整数。你有办法在O(n)时间内完成吗？

## 注意：
本题相对书上原题稍作改动

## 示例 1：
```
输入：[3,0,1]
输出：2
```

## 示例 2：
```
输入：[9,6,4,2,3,5,7,0,1]
输出：8
```

## 题解：
### Rust
```rust
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ret = 0;
        for i in 0..=n {
            ret ^= i as i32;
        }
        for i in 0..n {
            ret ^= nums[i];
        }
        ret
    }
}
```

### Go
```go
package main

func missingNumber(nums []int) int {
	n := len(nums)
	ret := 0
	for i := 0; i <= n; i++ {
		ret ^= i
	}
	for i := 0; i < n; i++ {
		ret ^= nums[i]
	}
	return ret
}

```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
var missingNumber = function(nums) {
    let n = nums.length
    let ret = 0
    for (let i = 0; i <= n; ++i) {
        ret ^= i
    }
    for (let i = 0; i < n; ++i) {
        ret ^= nums[i]
    }
    return ret
};
```

### Python
```python

class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        n = len(nums)
        ret = 0
        for i in range(n+1):
            ret ^= i
        for i in range(n):
            ret ^= nums[i]
        return ret
```

### C++
```c++
class Solution {
public:
    int missingNumber(vector<int>& nums) {
        int n = nums.size();
        int ret = 0;
        for (int i = 0; i <= n; ++i) {
            ret ^= i;
        }
        for (int i = 0; i < n; ++i) {
            ret ^= nums[i];
        }
        return ret;
    }
};

```