# 面试题 05.06. 整数转换
整数转换。编写一个函数，确定需要改变几个位才能将整数A转成整数B。

## 示例1:
```
输入：A = 29 （或者0b11101）, B = 15（或者0b01111）
输出：2
```

## 示例2:
```
输入：A = 1，B = 2
输出：2
```

## 提示:

- A，B范围在[-2147483648, 2147483647]之间

## 题解：
### Rust
```rust
impl Solution {
    pub fn convert_integer(a: i32, b: i32) -> i32 {
        let (mut c, mut count) = (a ^ b, 0);
        for _ in 0..32 {
            if c & 1 == 1 {
                count += 1;
            }
            c >>= 1;
        }
        count
    }
}

```

### Go
```go
package main

func convertInteger(A int, B int) int {
	C := A ^ B
	count := 0
	for i := 0; i < 32; i++ {
		if (C & 1) == 1 {
			count++
		}
		C >>= 1
	}
	return count
}

```

### JavaScript
```javascript
/**
 * @param {number} A
 * @param {number} B
 * @return {number}
 */
var convertInteger = function(A, B) {
    let C = A ^ B
    let count = 0
    for (let i = 0; i < 32; ++i) {
        if ((C&1) == 1) {
            count++
        }
        C >>= 1
    }
    return count
};

```

### Python
```python
class Solution:
    def convertInteger(self, A: int, B: int) -> int:
        C = A ^ B
        count = 0
        for i in range(32):
            if (C&1) == 1:
                count += 1
            C >>= 1
        return count
    
```

### C++
```c++
class Solution {
public:
    int convertInteger(int A, int B) {
        int C = A ^ B;
        int count = 0;
        for (int i = 0; i < 32; ++i) {
            if ((C & 1) == 1) {
                count++;
            }
            C >>= 1;
        }
        return count;
    }
};

```