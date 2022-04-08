# 剑指 Offer 56 - II. 数组中数字出现的次数 II

在一个数组 nums 中除一个数字只出现一次之外，其他数字都出现了三次。请找出那个只出现一次的数字。



## 示例 1：
```
输入：nums = [3,4,3,3]
输出：4
```


## 示例 2：
```
输入：nums = [9,1,7,9,7,9,7]
输出：1
```

## 限制：

- 1 <= nums.length <= 10000
- 1 <= nums[i] < 2^31

## 题解：
### Rust
```rust
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut bits = vec![0; 32];
        let mut mask = 1;
        for i in 0..32 {
            for j in 0..n {
                if (nums[j] & mask) != 0 {
                    bits[i] = (bits[i] + 1) % 3;
                }
            }
            mask <<= 1;
        }
        let mut result = 0;
        mask = 1;
        for i in 0..32 {
            if bits[i] == 1 {
                result += mask;
            }
            mask <<= 1;
        }
        result
    }
}

```

### Go
```go
package main

func singleNumber(nums []int) int {
	n := len(nums)
	bits := make([]int, 32)
	mask := 1
	for i := 0; i < 32; i++ {
		for j := 0; j < n; j++ {
			if (nums[j] & mask) != 0 {
				bits[i] = (bits[i] + 1) % 3
			}
		}
		mask <<= 1
	}
	result := 0
	mask = 1
	for i := 0; i < 32; i++ {
		if bits[i] == 1 {
			result += mask
		}
		mask <<= 1
	}
	return result
}
```

### JavaScript
```javascript

/**
 * @param {number[]} nums
 * @return {number}
 */
var singleNumber = function(nums) {
    let n = nums.length
    let bits = new Array(32).fill(0)
    let mask = 1
    for (let i = 0; i < 32; ++i) {
        for (let j = 0; j < n; ++j) {
            if ((nums[j] & mask) != 0) {
                bits[i] = (bits[i] + 1) % 3
            }
        }
        mask <<= 1
    }
    let result = 0
    mask = 1
    for (let i = 0; i < 32; ++i) {
        if (bits[i] == 1) {
            result += mask
        }
        mask <<= 1
    }
    return result
};
```

### Python
```python
class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        n = len(nums)
        bits = [0] * 32
        mask = 1
        for i in range(32):
            for j in range(n):
                if (nums[j]&mask) != 0:
                    bits[i] = (bits[i] + 1) % 3
            mask <<= 1
        result = 0
        mask = 1
        for i in range(32):
            if bits[i] == 1:
                result += mask
            mask <<= 1
        return result
```

### C++
```c++
class Solution {
public:
    int singleNumber(vector<int>& nums) {
        int n = nums.size();
        vector<int> bits(32);
        long mask = 1;
        for (int i = 0; i < 32; ++i) {
            for (int j = 0; j < n; ++j) {
                if ((nums[j] & mask) != 0) {
                    bits[i] = (bits[i] + 1) % 3;
                }
            }
            mask <<= 1;
        }
        int result = 0;
        mask = 1;
        for (int i = 0; i < 32; ++i) {
            if (bits[i] == 1) {
                result += mask;
            }
            mask <<= 1;
        }
        return result;;
    }
};

```