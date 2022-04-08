# 剑指 Offer 56 - I. 数组中数字出现的次数

一个整型数组 nums 里除两个数字之外，其他数字都出现了两次。请写程序找出这两个只出现一次的数字。要求时间复杂度是O(n)，空间复杂度是O(1)。

## 示例 1：

```
输入：nums = [4,1,4,6]
输出：[1,6] 或 [6,1]
```

## 示例 2：
```
输入：nums = [1,2,10,4,1,4,3,3]
输出：[2,10] 或 [10,2]
```

## 限制：
- 2 <= nums.length <= 10000

## 题解：
### Rust
```rust
impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let (mut xor_result, n) = (0, nums.len());
        for i in 0..n {
            xor_result ^= nums[i];
        }
        let mut tag = 1;
        while (xor_result & tag) == 0 {
            tag <<= 1;
        }
        let (mut a, mut b) = (0, 0);
        for i in 0..n {
            if nums[i] & tag == 0 {
                a ^= nums[i];
            } else {
                b ^= nums[i];
            }
        }
        vec![a, b]
    }
}
```

### Go
```go
package main

func singleNumbers(nums []int) []int {
	xorRexult := 0
	n := len(nums)
	for i := 0; i < n; i++ {
		xorRexult ^= nums[i]
	}
	tag := 1
	for (xorRexult & tag) == 0 {
		tag = tag << 1
	}
	a := 0
	b := 0
	for i := 0; i < n; i++ {
		if (nums[i] & tag) == 0 {
			a ^= nums[i]
		} else {
			b ^= nums[i]
		}
	}
	return []int{a, b}
}

```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {number[]}
 */
var singleNumbers = function(nums) {
    let xorResult = 0
    let n = nums.length
    for (let i = 0; i < n; ++i) {
        xorResult ^= nums[i]
    }
    let tag = 1
    while ((xorResult & tag) == 0) {
        tag = tag << 1
    }
    let a = 0
    let b = 0
    for (let i = 0; i < n; ++i) {
        if ((nums[i] & tag) == 0) {
            a ^= nums[i]
        } else {
            b ^= nums[i]
        }
    }
    return [a, b]
};

```

### Python
```python
class Solution:
    def singleNumbers(self, nums: List[int]) -> List[int]:
        xorResult = 0
        n = len(nums)
        for i in range(n):
            xorResult ^= nums[i]
        tag = 1
        while xorResult & tag == 0:
            tag = tag << 1
        a = 0
        b = 0
        for i in range(n):
            if (nums[i] & tag) == 0:
                a ^= nums[i]
            else:
                b ^= nums[i]
        return [a,b]
```

### C++
```c++
class Solution {
public:
    vector<int> singleNumbers(vector<int>& nums) {
        int xorResult = 0;
        int n = nums.size();
        for (int i = 0; i < n; ++i) {
            xorResult ^= nums[i];
        }
        int tag = 1;
        while ((xorResult & tag) == 0) {
            tag = tag << 1;
        }
        int a = 0;
        int b = 0;
        for (int i = 0; i < n; ++i) {
            if ((nums[i] & tag) == 0) {
                a ^= nums[i];
            } else {
                b ^= nums[i];
            }
        }
        return {a, b};
    }
};

```