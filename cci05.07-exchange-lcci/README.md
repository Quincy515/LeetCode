# 面试题 05.07. 配对交换
配对交换。编写程序，交换某个整数的奇数位和偶数位，尽量使用较少的指令（也就是说，位0与位1交换，位2与位3交换，以此类推）。

## 示例1:
```
输入：num = 2（或者0b10）
输出 1 (或者 0b01)
```

## 示例2:
```
输入：num = 3
输出：3
```

## 提示:

- num的范围在[0, 2^30 - 1]之间，不会发生整数溢出。

## 题解：
### Rust
```rust

impl Solution {
    pub fn exchange_bits(num: i32) -> i32 {
        let mut ret = 0;
        let mut i = 0;
        while i <= 30 {
            // 奇位与偶位
            let a1 = num & (1 << i);
            let b1 = num & (1 << (i + 1));
            // 如果是 1，则加上交换位；0 则加上也没用
            if a1 != 0 {
                ret |= 1 << (i + 1);
            }
            if b1 != 0 {
                ret |= 1 << i;
            }
            i += 2;
        }

        ret
    }
}
```

### Go
```go
package main

func exchangeBits(num int) int {
	ret := 0
	for i := 0; i <= 30; i += 2 {
		a1 := (num & (1 << i))
		b1 := (num & (1 << (i + 1)))
		if a1 != 0 {
			ret |= (1 << (i + 1))
		}
		if b1 != 0 {
			ret |= (1 << i)
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
var exchangeBits = function(num) {
    let ret = 0
    for (let i = 0; i <= 30; i+=2) {
        let a1 = (num & (1 << i))
        let b1 = (num & (1 << (i+1)))
        if (a1 != 0) {
            ret |= (1 << (i+1))
        }
        if (b1 != 0) {
            ret |= (1 << i)
        }
    }
    return ret
};

```

### Python
```python

class Solution:
    def exchangeBits(self, num: int) -> int:
        ret = 0
        for i in range(0, 30, 2):
            a1 = (num & (1 << i))
            b1 = (num & (1 << (i+1)))
            if a1 != 0:
                ret |= (1<<(i+1))
            if b1 != 0:
                ret |= (1<<i)
        return ret

```

### C++
```c++
class Solution {
public:
    int exchangeBits(int num) {
        int ret = 0;
        for (int i = 0; i <= 30; i += 2) {
            int a1 = (num & (1 << i));
            int b1 = (num & (1 << (i + 1)));
            if (a1 != 0) ret |= (1 << (i + 1));
            if (b1 != 0) ret |= (1 << i);
        }
        return ret;
    }
};

```