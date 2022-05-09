# 461. 汉明距离
两个整数之间的[汉明距离](https://baike.baidu.com/item/%E6%B1%89%E6%98%8E%E8%B7%9D%E7%A6%BB)指的是这两个数字对应二进制位不同的位置的数目。

给出两个整数 ```x``` 和 ```y```，计算它们之间的汉明距离。

#### 注意:
0 ≤ ```x```, ```y``` < 2<sup>31</sup>.

#### 示例:
<pre>
<strong>输入:</strong> x = 1, y = 4
<strong>输出:</strong> 2
<strong>解释:</strong>
1   (0 0 0 1)
4   (0 1 0 0)
       ↑   ↑
上面的箭头指出了对应二进制位不同的位置。
</pre>

## 思路：

对于两个数来说，如果相同位置的二进制位相同，则对应汉明距离为0；否则为1；和异或的性质不谋而合，所以可以将两个数异或以后，求异或结果的 **位1的个数**

## 题解：

```rust
impl Solution {
    fn hammingWeight (mut n: i32) -> i32 {
        let mut result = 0;
        while n != 0 {
            result += n & 1;
            n >>= 1;
        }
        result
    }

    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        return Self::hammingWeight(x ^ y);
    }
}
```



### Rust
```rust

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let (r, mut mask, mut count) = ((x ^ y) as i64, 1i64, 0);
        for _ in 0..31 {
            if (r & mask) != 0 {
                count += 1;
            }
            mask *= 2;
        }
        count
    }
}
```

### Go
```go
package main

func hammingDistance(x int, y int) int {
	r := x ^ y
	mask := 1
	count := 0
	for i := 0; i < 31; i++ {
		if r&mask != 0 {
			count++
		}
		mask *= 2
	}
	return count
}

```

### JavaScript
```javascript
/**
 * @param {number} x
 * @param {number} y
 * @return {number}
 */
var hammingDistance = function(x, y) {
    let r = x ^ y
    let mask = 1
    let count = 0
    for (let i = 0; i < 31; i++) {
        if ((r & mask) != 0) {
            count++
        }
        mask *= 2
    }
    return count
}
```

### Python
```python
class Solution:
    def hammingDistance(self, x: int, y: int) -> int:
        r = x ^ y
        mask = 1
        count = 0
        for i in range(31):
            if (r & mask) != 0:
                count += 1
            mask *= 2
        return count
    
```

### C++
```c++
class Solution {
public:
    int hammingDistance(int x, int y) {
        int r = x ^ y;
        long mask = 1;
        int count = 0;
        for (int i = 0; i < 31; ++i) {
            if ((r & mask) != 0) {
                count++;
            }
            mask *= 2;
        }
        return count;
    }
};

```