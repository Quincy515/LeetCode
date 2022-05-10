# 231. 2的幂
给定一个整数，编写一个函数来判断它是否是 2 的幂次方。

#### 示例 1:
<pre>
<strong>输入:</strong> 1
<strong>输出:</strong> true
<strong>解释:</strong> 2<sup>0</sup> = 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 16
<strong>输出:</strong> true
<strong>解释:</strong> 2<sup>4</sup> = 16
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> false
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> 4
<strong>输出:</strong> true
</pre>


#### 示例 5:
<pre>
<strong>输入:</strong> 5
<strong>输出:</strong> false
</pre>

#### 示例 6:
<pre>
<strong>输入:</strong> 218
<strong>输出:</strong> false
</pre>

## 题解：
### Rust
```rust
impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        while n != 0 {
            if (n & 1) == 1 {
                return (n >> 1) == 0;
            }
            n >>= 1;
        }
        false
    }
}
```

### Go
```go
package main

func isPowerOfTwo(n int) bool {
	for n != 0 {
		if (n & 1) == 1 {
			if (n >> 1) == 0 {
				return true
			} else {
				return false
			}
		}
		n >>= 1
	}
	return false
}
```

### JavaScript
```javascript
/**
 * @param {number} n
 * @return {boolean}
 */
var isPowerOfTwo = function(n) {
    while (n != 0) {
        if ((n & 1) == 1) {
            if ((n >> 1) == 0) {
                return true
            } else {
                return false
            }
        }
        n >>= 1
    }
    return false
};
```

### Python
```python
class Solution:
    def isPowerOfTwo(self, n: int) -> bool:
        while n != 0:
            if (n & 1) == 1:
                if (n >> 1) == 0:
                    return True
            else:
                return False
            n >>= 1
        return False
```

### C++
```c++
class Solution {
public:
    bool isPowerOfTwo(int n) {
        while (n != 0) {
            if ((n & 1) == 1) {
                if ((n >> 1) == 0) {
                    return true;
                } else {
                    return false;
                }
            }
            n >>= 1;
        }
        return false;
    }
};
```