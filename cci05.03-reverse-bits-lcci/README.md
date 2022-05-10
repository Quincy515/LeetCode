# 面试题 05.03. 翻转数位
给定一个32位整数 num，你可以将一个数位从0变为1。请编写一个程序，找出你能够获得的最长的一串1的长度。

## 示例 1：

```
输入: num = 1775(110111011112)
输出: 8
```

## 示例 2：
```
输入: num = 7(01112)
输出: 4
```

## 题解：
### Rust
```rust

impl Solution {
    pub fn reverse_bits(mut num: i32) -> i32 {
        if num == 0 {
            return 1;
        }
        let mut nums = vec![0; 32];
        for i in 0..32 {
            nums[i] = num & 1;
            num >>= 1;
        }
        let mut left_one_counts = vec![0; 32];
        let mut count = 0;
        for i in 0..32 {
            left_one_counts[i] = count;
            if nums[i] == 1 {
                count += 1;
            } else {
                count = 0;
            }
        }
        let mut right_one_counts = vec![0; 32];
        let mut count = 0;
        for i in (0..=31).rev() {
            right_one_counts[i] = count;
            if nums[i] == 1 {
                count += 1;
            } else {
                count = 0;
            }
        }

        let mut ret = 0;
        for i in 0..32 {
            if ret < left_one_counts[i] + right_one_counts[i] + 1 {
                ret = left_one_counts[i] + right_one_counts[i] + 1;
            }
        }
        ret
    }
}

```

### Go
```go

package main

func reverseBits(num int) int {
	if num == 0 {
		return 1
	}
	nums := make([]int, 32)
	for i := 0; i < 32; i++ {
		nums[i] = (num & 1)
		num >>= 1
	}
	leftOneCounts := make([]int, 32)
	count := 0
	for i := 0; i < 32; i++ {
		leftOneCounts[i] = count
		if nums[i] == 1 {
			count++
		} else {
			count = 0
		}
	}
	rightOneCounts := make([]int, 32)
	count = 0
	for i := 31; i >= 0; i-- {
		rightOneCounts[i] = count
		if nums[i] == 1 {
			count++
		} else {
			count = 0
		}
	}
	ret := 0
	for i := 0; i < 32; i++ {
		if ret < leftOneCounts[i]+rightOneCounts[i]+1 {
			ret = leftOneCounts[i] + rightOneCounts[i] + 1
		}
	}
	return ret
}
```

### JavaScript
```javascript

/**
 * @param {number} num
 * @return {number}
 */
var reverseBits = function(num) {
    if (num == 0) {
        return 1
    }
    let nums = new Array(32)
    for (let i = 0; i < 32; ++i) {
        nums[i] = (num&1)
        num >>= 1
    }
    let leftOneCounts = new Array(32)
    let count = 0
    for (let i = 0; i < 32; ++i) {
        leftOneCounts[i] = count
        if (nums[i] == 1) {
            count++
        } else {
            count = 0
        }
    }
    let rightOneCounts = new Array(32)
    count = 0
    for (let i = 31; i >= 0; --i) {
        rightOneCounts[i] = count
        if (nums[i] == 1) {
            count++
        } else {
            count = 0
        }
    }
    let ret = 0
    for (let i = 0; i < 32; ++i) {
        if (ret < leftOneCounts[i] + rightOneCounts[i] + 1) [
            ret = leftOneCounts[i] + rightOneCounts[i] + 1
        ]
    }
    return ret
};
```

### Python
```python
class Solution:
    def reverseBits(self, num: int) -> int:
        if num == 0:
            return 1
        nums = [None] * 32
        for i in range(32):
            nums[i] = num & 1
            num >>= 1
        leftOneCounts = [0] * 32
        count = 0
        for i in range(32):
            leftOneCounts[i] = count
            if nums[i] == 1:
                count += 1
            else:
                count = 0
        rightOneCounts = [0] * 32
        count = 0
        for i in range(31, -1,-1):
            rightOneCounts[i] = count
            if nums[i] == 1:
                count += 1
            else:
                count = 0
        ret = 0
        for i in range(32):
            if ret < leftOneCounts[i] + rightOneCounts[i] + 1:
                ret = leftOneCounts[i] + rightOneCounts[i] + 1
        return ret

```

### C++
```c++
class Solution {
public:
    int reverseBits(int num) {
        if (num == 0) return 1;
        std::vector<int> nums(32);
        for (int i = 0; i < 32; ++i) {
            nums[i] = (num & 1);
            num >>= 1;
        }
        vector<int> leftOneCounts(32);
        int count = 0;
        for (int i = 0; i < 32; ++i) {
            leftOneCounts[i] = count;
            if (nums[i] == 1) {
                count++;
            } else {
                count = 0;
            }
        }
        vector<int> rightOneCounts(32);
        count = 0;
        for (int i = 31; i >= 0; --i) {
            rightOneCounts[i] = count;
            if (nums[i] == 1) {
                count++;
            } else {
                count = 0;
            }
        }
        int ret = 0;
        for (int i = 0; i < 32; ++i) {
            if (ret < leftOneCounts[i] + rightOneCounts[i] + 1) {
                ret = leftOneCounts[i] + rightOneCounts[i] + 1;
            }
        }
        return ret;
    }
}
```