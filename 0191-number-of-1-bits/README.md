# 191. 位1的个数
编写一个函数，输入是一个无符号整数，返回其二进制表达式中数字位数为 ‘1’ 的个数（也被称为[汉明重量](https://baike.baidu.com/item/%E6%B1%89%E6%98%8E%E9%87%8D%E9%87%8F)）。

#### 示例 1:
<pre>
<strong>输入:</strong> 00000000000000000000000000001011
<strong>输出:</strong> 3
<strong>解释:</strong> 输入的二进制串 <strong>00000000000000000000000000001011</strong> 中，共有三位为 '1'。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 00000000000000000000000010000000
<strong>输出:</strong> 1
<strong>解释:</strong> 输入的二进制串 <strong>00000000000000000000000010000000</strong> 中，共有一位为 '1'。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 11111111111111111111111111111101
<strong>输出:</strong> 31
<strong>解释:</strong> 输入的二进制串 <strong>11111111111111111111111111111101</strong> 中，共有 31 位为 '1'。
</pre>

#### 提示:
* 请注意，在某些语言（如 Java）中，没有无符号整数类型。在这种情况下，输入和输出都将被指定为有符号整数类型，并且不应影响您的实现，因为无论整数是有符号的还是无符号的，其内部的二进制表示形式都是相同的。
* 在 Java 中，编译器使用[二进制补码](https://baike.baidu.com/item/%E8%A1%A5%E7%A0%81/6854613?fromtitle=%E4%BA%8C%E8%BF%9B%E5%88%B6%E8%A1%A5%E7%A0%81&fromid=5295284)记法来表示有符号整数。因此，在上面的 **示例 3** 中，输入表示有符号整数 ```-3```。

#### 进阶:
如果多次调用这个函数，你将如何优化你的算法？

## 题解：
### Rust
```rust

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let (mut one_count, mut mask) = (0, 1);
        for i in 0..32 {
            if (n & mask) != 0 {
                one_count += 1;
            }
            mask <<= 1;
        }
        one_count
    }
    pub fn hammingWeight2(mut n: u32) -> i32 {
        let mut one_count = 0;
        let mut i = 1;
        while n != 0 {
            if n & 1 == 1 {
                one_count += 1;
            }
            n >>= 1;
        }
        one_count
    }
}
```

### Go
```go
package main

func hammingWeight(num uint32) int {
	oneCount := 0
	var mask uint32 = 1
	for i := 0; i < 32; i++ {
		if num&mask != 0 {
			oneCount++
		}
		mask <<= 1
	}
	return oneCount
}
func hammingWeight(num uint32) int {
	oneCount := 0
	for num != 0 {
		if num&1 == 1 {
			oneCount++
		}
		num >>= 1
	}
	return oneCount
}

```

### JavaScript
```javascript
/**
 * @param {number} n - a positive integer
 * @return {number}
 */
var hammingWeight = function(n) {
    let oneCount = 0
    let mask = 1
    for (let i = 0; i < 32; i++) {
        if ((n & mask) != 0) {
            oneCount++
        }
        mask <<= 1
    }
    return oneCount
}
```

### Python
```python
class Solution:
    def hammingWeight(self, n: int) -> int:
        oneCount = 0
        mask = 1
        for i in range(32):
            if n & mask != 0:
                oneCount += 1
            mask <<= 1
        return oneCount

class Solution:
    def hammingWeight(self, n: int) -> int:
        oneCount = 0
        i = 1
        while n != 0:
            if (n & 1) == 1:
                oneCount += 1
            n >>= 1
        return oneCount

```

### C++
```c++
class Solution {
public:
    int hammingWeight(uint32_t n) {
        int oneCount = 0;
        long mask = 1;
        for (int i = 0; i < 32; ++i) {
            if ((n & mask) != 0) {
                oneCount++;
            }
            mask <<= 1;
        }
        return oneCount;
    }
};

class Solution {
public:
    int hammingWeight(uint32_t n) {
        int oneCount = 0;
        int i = 1;
        while (n != 0) {
            if ((n & 1) == 1) {
                oneCount++;
            }
            n >>= 1;
        }
        return oneCount;
    }
};

```