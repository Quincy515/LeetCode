# 面试题 16.01. 交换数字

编写一个函数，不用临时变量，直接交换numbers = [a, b]中a与b的值。

## 示例：
```
输入: numbers = [1,2]
输出: [2,1]
```

## 提示：

- numbers.length == 2
- -2147483647 <= numbers[i] <= 2147483647

## 题解：
### Rust
```rust
impl Solution {
    pub fn swap_numbers(mut numbers: Vec<i32>) -> Vec<i32> {
        if numbers[0] == numbers[1] {
            return numbers;
        }
        numbers.swap(0, 1);
        numbers
    }
}
```

### Go
```go
package main

func swapNumbers(numbers []int) []int {
	if numbers[0] == numbers[1] {
		return numbers
	}
	numbers[0] ^= numbers[1]
	numbers[1] ^= numbers[0]
	numbers[0] ^= numbers[1]
	return numbers
}
```

### JavaScript
```javascript
/**
 * @param {number[]} numbers
 * @return {number[]}
 */
var swapNumbers = function(numbers) {
        if (numbers[0] == numbers[1]) {
            return numbers
        }
        numbers[0] ^= numbers[1]
        numbers[1] ^= numbers[0]
        numbers[0] ^= numbers[1]
        return numbers
    };

```

### Python
```python
class Solution:
    def swapNumbers(self, numbers: List[int]) -> List[int]:
        if numbers[0] == numbers[1]:
            return numbers
        numbers[0] ^= numbers[1]
        numbers[1] ^= numbers[0]
        numbers[0] ^= numbers[1]
        return numbers
```

### C++
```c++
class Solution {
public:
    vector<int> swapNumbers(vector<int>& numbers) {
        if (numbers[0] == numbers[1]) return numbers;
        numbers[0] ^= numbers[1];
        numbers[1] ^= numbers[0];
        numbers[0] ^= numbers[1];
        return numbers;
    }
};

```