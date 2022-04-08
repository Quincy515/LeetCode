# 面试题 05.01. 插入

给定两个整型数字 N 与 M，以及表示比特位置的 i 与 j（i <= j，且从 0 位开始计算）。

编写一种方法，使 M 对应的二进制数字插入 N 对应的二进制数字的第 i ~ j 位区域，不足之处用 0 补齐。具体插入过程如图所示。


![](https://pic.leetcode-cn.com/1610104070-NuLVQi-05.01.gif)

题目保证从 i 位到 j 位足以容纳 M， 例如： M = 10011，则 i～j 区域至少可容纳 5 位。

## 示例1:
```
输入：N = 1024(10000000000), M = 19(10011), i = 2, j = 6
输出：N = 1100(10001001100)
```

## 示例2:
```
输入： N = 0, M = 31(11111), i = 0, j = 4
输出：N = 31(11111)
```

## 题解：
### Rust
```rust
impl Solution {
    pub fn insert_bits(n: i32, m: i32, i: i32, j: i32) -> i32 {
        let (mut nbits, mut mbits) = (vec![0; 32], vec![0; 32]);
        let mut mask = 1;
        for k in 0..32 {
            if (n & mask) != 0 {
                nbits[k] = 1;
            }
            mask <<= 1;
        }
        mask = 1;
        for k in 0..32 {
            if (m & mask) != 0 {
                mbits[k] = 1;
            }
            mask <<= 1;
        }
        for k in i..=j {
            nbits[k as usize] = mbits[(k - i) as usize];
        }
        mask = 1;
        let mut ret = 0;
        for k in 0..32 {
            ret += nbits[k] * mask;
            mask <<= 1;
        }
        ret
    }
}
```

### Go
```go
package main

func insertBits(N int, M int, i int, j int) int {
	nbits := make([]int, 32)
	mbits := make([]int, 32)
	mask := 1
	for k := 0; k < 32; k++ {
		if N&mask != 0 {
			nbits[k] = 1
		}
		mask <<= 1
	}
	mask = 1
	for k := 0; k < 32; k++ {
		if (M & mask) != 0 {
			mbits[k] = 1
		}
		mask <<= 1
	}
	for k := i; k <= j; k++ {
		nbits[k] = mbits[k-i]
	}
	mask = 1
	ret := 0
	for k := 0; k < 32; k++ {
		ret += (nbits[k] * mask)
		mask <<= 1
	}
	return ret
}

```

### JavaScript
```javascript
/**
 * @param {number} N
 * @param {number} M
 * @param {number} i
 * @param {number} j
 * @return {number}
 */
var insertBits = function(N, M, i, j) {
    let nbits = new Array(32).fill(0)
    let mbits = new Array(32).fill(0)
    let mask = 1
    for (let k = 0; k < 32; ++k) {
        if ((N&mask) != 0) {
            nbits[k] = 1
        }
        mask <<= 1
    }
    mask = 1
    for (let k = 0; k < 32; ++k) {
        if ((M&mask) != 0) {
            mbits[k] = 1
        }
        mask <<= 1
    }
    for (let k = i; k <= j; ++k) {
        nbits[k] = mbits[k-i]
    }
    mask = 1
    let ret = 0
    for (let k = 0; k < 32; ++k) {
        ret += (nbits[k] * mask)
        mask <<= 1
    }
    return ret
};

```

### Python
```python
class Solution:
    def insertBits(self, N: int, M: int, i: int, j: int) -> int:
        nbits = [0] * 32
        mbits = [0] * 32
        nbitsNum = 0
        mbitsNum = 0
        mask = 1
        for k in range(32):
            if (N & mask) != 0:
                nbits[k] = 1
            mask <<=1
        mask = 1
        for k in range(32):
            if (M&mask) != 0:
                mbits[k] = 1
            mask <<= 1
        for k in range(i,j+1):
            nbits[k] = mbits[k-i]
        mask = 1
        ret = 0
        for k in range(32):
            ret += (nbits[k] * mask)
            mask <<= 1
        return ret

```

### C++
```c++
class Solution {
public:
    int insertBits(int N, int M, int i, int j) {
        vector<int> nbits(32);
        vector<int> mbits(32);
        int nbitsNum = 0;
        int mbitsNum = 0;
        long mask = 1;
        for (int k = 0; k < 32; ++k) {
            if ((N & mask) != 0) {
                nbits[k] = 1;
            }
            mask <<= 1;
        }
        mask = 1L;
        for (int k = 0; k < 32; ++k) {
            if ((M & mask) != 0) {
                mbits[k] = 1;
            }
            mask <<= 1;
        }
        for (int k = i; k <= j; ++k) {
            nbits[k] = mbits[k - i];
        }
        mask = 1;
        int ret = 0;
        for (int k = 0; k < 32; ++k) {
            ret += (nbits[k] * mask);
            mask <<= 1;
        }
        return ret;
    }
};

```