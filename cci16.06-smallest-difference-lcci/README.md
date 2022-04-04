# 面试题 16.06. 最小差
给定两个整数数组a和b，计算具有最小差绝对值的一对数值（每个数组中取一个值），并返回该对数值的差



## 示例：
```
输入：{1, 3, 15, 11, 2}, {23, 127, 235, 19, 8}
输出：3，即数值对(11, 8)
```

## 提示：

- 1 <= a.length, b.length <= 100000
- -2147483648 <= a[i], b[i] <= 2147483647
- 正确结果在区间 [0, 2147483647] 内

## 题解：
### Rust
```rust
impl Solution {
    pub fn smallest_difference(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
        a.sort();
        b.sort();
        let (n, m) = (a.len(), b.len());
        let mut min_ret = i64::MAX;
        let (mut i, mut j) = (0, 0);
        while i < n && j < m {
            if a[i] >= b[j] {
                min_ret = i64::abs(min_ret.min(a[i] as i64 - b[j] as i64));
                j += 1;
            } else {
                min_ret = i64::abs(min_ret.min(b[j] as i64 - a[i] as i64));
                i += 1;
            }
        }
        min_ret as i32
    }
}
```

### Go
```go
package main

import (
	"math"
	"sort"
)

func smallestDifference(a []int, b []int) int {
	sort.Ints(a)
	sort.Ints(b)
	n := len(a)
	m := len(b)
	var minRet int64 = math.MaxInt64
	i := 0
	j := 0
	for i < n && j < m {
		if a[i] >= b[j] {
			minRet = int64(math.Min(float64(minRet), float64(a[i]-b[j])))
			j++
		} else {
			minRet = int64(math.Min(float64(minRet), float64(b[j]-a[i])))
			i++
		}
	}
	return int(minRet)
}

```

### JavaScript
```javascript
/**
 * @param {number[]} a
 * @param {number[]} b
 * @return {number}
 */
var smallestDifference = function(a, b) {
    let sortFn = (i, j) => i - j
    a.sort(sortFn)
    b.sort(sortFn)
    let n = a.length
    let m = b.length
    let minRet = Number.MAX_SAFE_INTEGER
    let i = 0
    let j = 0
    while (i < n && j < m) {
        if (a[i] >= b[j]) {
            minRet = Math.min(minRet, a[i] - b[j])
            j++
        } else {
            minRet = Math.min(minRet, b[j] - a[i])
            i++
        }
    }
    return minRet
};
```

### Python
```python
class Solution:
    def smallestDifference(self, a: List[int], b: List[int]) -> int:
        a.sort()
        b.sort()
        n = len(a)
        m = len(b)
        minRet = float("inf")
        i = 0
        j = 0
        while i < n and j < m:
            if a[i] >= b[j]:
                minRet = min(minRet, a[i] - b[j])
                j += 1
            else:
                minRet = min(minRet, b[j] - a[i])
                i += 1
        return minRet
```

### C++
```c++
class Solution {
public:
    long res = LONG_MAX;
    int smallestDifference(vector<int>& a, vector<int>& b) {
        std::sort(a.begin(), a.end());
        std::sort(b.begin(), b.end());
        int n = a.size();
        int m = b.size();
        int i = 0;
        int j = 0;
        while (i < n && j < m) {
            if (a[i] >= b[j]) {
                res = std::min(res, (long)a[i] - b[j]);
                j++;
            } else {
                res = std::min(res, (long)b[j] - a[i]);
                i++;
            }
        }
        return (int)res;
    }
};
```