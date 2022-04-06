# 42. 接雨水
    给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。



## 示例 1：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/22/rainwatertrap.png)

```
输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
输出：6
解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
```

## 示例 2：
```
输入：height = [4,2,0,3,2,5]
输出：9
```

## 提示：

- n == height.length
- 1 <= n <= 2 * 104
- 0 <= height[i] <= 105


## 题解：
### Rust
```rust
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut result = 0;
        // 遍历每个柱子 h, 查找它左边的最高柱子 lh, 和右边的最高柱子 rh
        // 柱子上能承载的雨水 = min(lh, rh) - h
        for i in 1..n - 1 {
            let mut lh = 0;
            for j in 0..i {
                // 左侧最高 lh
                if height[j] > lh {
                    lh = height[j];
                }
            }
            let mut rh = 0;
            for j in i + 1..n {
                // 右侧最高 rh
                if height[j] > rh {
                    rh = height[j];
                }
            }
            let mut carry = i32::min(lh, rh) - height[i];
            if carry < 0 {
                carry = 0;
            }
            result += carry;
        }
        result
    }
}

```

```rust
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        // 前缀 max
        let mut left_max = vec![0; n];
        let mut max = 0;
        for i in 0..n {
            left_max[i] = i32::max(max, height[i]);
            max = left_max[i];
        }
        // 后缀 max
        let mut right_max = vec![0; n];
        max = 0;
        for i in (0..=n - 1).rev() {
            right_max[i] = i32::max(max, height[i]);
            max = right_max[i];
        }
        // 每个柱子之上承载的雨水
        let mut result = 0;
        for i in 1..n - 1 {
            result += i32::min(left_max[i], right_max[i]) - height[i];
        }
        result

    }
}

```

### Go
```go
package main

import "math"

func trap(height []int) int {
	n := len(height)
	result := 0 //遍历每个柱子 n，查找它左边的最高柱子 lh，和有变得最高柱子 rh //柱子上能承载的雨水=min(lh,rh)-h
	for i := 1; i < n-1; i++ {
		lh := 0
		for j := 0; j < i; j++ { // 左侧最高 lh
			if height[j] > lh {
				lh = height[j]
			}
		}
		rh := 0
		for j := i + 1; j < n; j++ { // 右侧最高 rh
			if height[j] > rh {
				rh = height[j]
			}
		}
		carry := int(math.Min(float64(lh), float64(rh))) - height[i]
		if carry < 0 {
			carry = 0
		}
		result += carry
	}
	return result
}

//解法 2
func trap(height []int) int {
	n := len(height) // 前缀 max
	leftMax := make([]int, n)
	max := 0
	for i := 0; i < n; i++ {
		leftMax[i] = int(math.Max(float64(max), float64(height[i])))
		max = leftMax[i]
	} // 后缀 max
	rightMax := make([]int, n)
	max = 0
	for i := n - 1; i >= 0; i-- {
		rightMax[i] = int(math.Max(float64(max), float64(height[i])))
		max = rightMax[i]
	} // 每个柱子上承载的雨水
	result := 0
	for i := 1; i < n-1; i++ {
		result += int(math.Min(float64(leftMax[i]), float64(rightMax[i]))) - height[i]
	}
	return result
}

```

### JavaScript
```javascript
/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function(height) {
    let n = height.length
    let result = 0
    for (let i = 1; i < n-1; ++i) {
        let lh = 0
        for (let j = 0; j < i; ++j) {
            if (height[j] > lh) {
                lh = height[j]
            }
        }
        let rh = 0
        for (let j = i+1; j < n; ++j) {
            if (height[j] > rh) {
                rh = height[j]
            }
        }
        let carry = Math.min(lh, rh) - height[i]
        if (carry < 0) {
            carry = 0
        }
        result += carry
    }
    return result
};
// 前缀/后缀统计解法
/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function(height) {
    let n = height.length

    let leftMax = new Array(n)
    let max = 0
    for (let i = 0; i < n; ++i) {
        leftMax[i] = Math.max(max, height[i])
        max = leftMax[i]
    }
    let rightMax = new Array(n)
    max = 0
    for (let i = n-1; i >= 0; --i) {
        rightMax[i] = Math.max(max, height[i])
        max = rightMax[i]
    }
    let result = 0
    for (let i = 1; i < n-1; i++) {
        result += Math.min(leftMax[i], rightMax[i]) - height[i]
    }
    return result
}
```

### Python
```python

class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        result = 0
        for i in range(1, n-1):
            lh = 0
            for j in range(i):
                if height[j] > lh:
                    lh = height[j]
            rh = 0
            for j in range(i+1,n):
                if height[j] > rh:
                    rh = height[j]
            carry = min(lh, rh) - height[i]
            if carry < 0:
                carry = 0
            result += carry
        return result

class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        leftMax = [None] * n
        curMax = 0
        for i in range(n):
            leftMax[i] = max(curMax, height[i])
            curMax = leftMax[i]
        rightMax = [None] * n
        curMax = 0
        for i in range(n-1,-1,-1):
            rightMax[i] = max(curMax, height[i])
            curMax = rightMax[i]
        result = 0
        for i in range(1,n-1):
            result += min(leftMax[i], rightMax[i]) - height[i]
        return result
```

### C++
```c++
class Solution {
public:
    int trap(vector<int>& height) {
        int n = height.size();
        int result = 0;
        for (int i = 1; i < n - 1; ++i) {
            int lh = 0;
            for (int j = 0; j < i; ++j) {
                if (height[j] > lh) {
                    lh = height[j];
                }
            }
            int rh = 0;
            for (int j = i + 1; j < n; ++j) {
                if (height[j] > rh) {
                    rh = height[j];
                }
            }
            int carry = std::min(lh, rh) - height[i];
            if (carry < 0) carry = 0;
            result += carry;
        }
        return result;
    }
};

class Solution {
public:
    int trap(vector<int>& height) {
        int n = height.size();
        // 前缀
        vector<int> leftMax(n);
        int max = 0;
        for (int i = 0; i < n ; ++i) {
            leftMax[i] = std::max(max, height[i]);
            max = leftMax[i];
        }
        // 后缀
        vector<int> rightMax(n);
        max = 0;
        for (int i = n - 1; i >= 0 ; --i) {
            rightMax[i] = std::max(max, height[i]);
            max = rightMax[i];
        }
        // 每个柱⼦之上承载的⾬⽔
        int result = 0;
        for (int i = 1; i < n - 1; ++i) {
            result += std::min(leftMax[i], rightMax[i]) - height[i];
        }
        return result;
    }
}
```