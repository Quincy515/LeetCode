# 剑指 Offer 46. 把数字翻译成字符串

给定一个数字，我们按照如下规则把它翻译为字符串：0 翻译成 “a” ，1 翻译成 “b”，……，11 翻译成 “l”，……，25 翻译成 “z”。一个数字可能有多个翻译。请编程实现一个函数，用来计算一个数字有多少种不同的翻译方法。


## 示例 1:
```
输入: 12258
输出: 5
解释: 12258有5种不同的翻译，分别是"bccfi", "bwfi", "bczi", "mcfi"和"mzi"
```

## 提示：

- 0 <= num < 231

## 题解
### Rust
```rust
impl Solution {
    pub fn translate_num(mut num: i32) -> i32 {
        if num < 9 {
            return 1;
        }
        // 把十进制转化成数字数组
        let mut digitlist = vec![];
        while num != 0 {
            digitlist.push(num % 10);
            num /= 10;
        }
        let n = digitlist.len();
        let mut digits = vec![0; n];
        for i in 0..n {
            digits[i] = digitlist[n - i - 1];
        }

        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        // dp[i] 表示 digits[0~i-1]（长度为 i）转化为字母有多少种方法
        // dp[i] = dp[i-1] + dp[i-2]（digits[i-2, i-1] 可翻译）
        // dp[i] = dp[i-1] (digits[i-2, i-1] 不可翻译)

        for i in 1..=n {
            dp[i] = dp[i - 1];
            if i >= 2 && Self::is_valid(digits[i - 2], digits[i - 1]) {
                dp[i] += dp[i - 2];
            }
        }

        dp[n]
    }

    fn is_valid(a: i32, b: i32) -> bool {
        if a == 1 {
            return true;
        }
        if a == 2 && b >= 0 && b <= 5 {
            return true;
        }
        false
    }
}
```

### Go
```go
package main

func translateNum(num int) int {
	if num <= 9 {
		return 1
	}
	//把十进制数转化成数字数组
	digitlist := make([]int, 0)
	for num != 0 {
		digitlist = append(digitlist, num%10)
		num /= 10
	}
	n := len(digitlist)
	digits := make([]int, n)
	for i := 0; i < n; i++ {
		digits[i] = digitlist[n-i-1]
	}
	dp := make([]int, n+1)
	dp[0] = 1
	for i := 1; i <= n; i++ {
		dp[i] = dp[i-1]
		if i-2 >= 0 && isValid(digits[i-2], digits[i-1]) {
			dp[i] += dp[i-2]
		}
	}
	return dp[n]
}
func isValid(a, b int) bool {
	if a == 1 {
		return true
	}
	if a == 2 && b >= 0 && b <= 5 {
		return true
	}
	return false
}

```

### JavaScript
```javascript
/**
 * @param {number} num
 * @return {number}
 */
var translateNum = function(num) {
    if (num <= 9) return 1
    let digitlist = []
    while (num != 0) {
        digitlist.push(num % 10)
        num = Math.floor(num/10)
    }
    let n = digitlist.length
    let digits = new Array(n)
    for (let i = 0; i < n; ++i) {
        digits[i] = digitlist[n-i-1]
    }
    let dp = new Array(n+1).fill(0)
    dp[0] = 1
    for (let i = 1; i <= n; ++i) {
        dp[i] = dp[i-1]
        if (i-2 >= 0 && isValid(digits[i-2], digits[i-1])) {
            dp[i] += dp[i-2]
        }
    }
    return dp[n]
};
var isValid = (a, b) => {
    if (a == 1) return true
    if (a == 2 && b>=0 && b<=5) return true
    return false
}
```

### Python
```python
class Solution:
    def translateNum(self, num: int) -> int:
        if num <= 9:
            return 1
        digitlist = []
        while num != 0:
            digitlist.append(num % 10)
            num = num // 10
        n = len(digitlist)
        digits = []
        for i in range(n):
            digits.append(digitlist[n-i-1])
        dp = [0] * (n + 1)
        dp[0] = 1
        for i in range(1,n+1):
            dp[i] = dp[i-1]
            if i - 2 >= 0 and self.isValid(digits[i-2], digits[i-1]):
                dp[i] += dp[i-2]
        return dp[n]

    def isValid(self,a,b):
        if a == 1:
            return True
        if a == 2 and 0 <= b <= 5:
            return True
        return False
    
```

### C++
```c++
class Solution {
public:
    int translateNum(int num) {
        if (num <= 9) return 1;
        std::vector<int> digitlist;
        while (num) {
            digitlist.push_back(num % 10);
            num /= 10;
        }
        int n = digitlist.size();
        vector<int> digits(n);
        for (int i = 0; i < n; ++i) {
            digits[i] = digitlist[n - i - 1];
        }
        vector<int> dp(n + 1);
        dp[0] = 1;
        for (int i = 1; i <= n; ++i) {
            dp[i] = dp[i - 1];
            if (i - 2 >= 0 && isValid(digits[i - 2], digits[i - 1])) {
                dp[i] += dp[i - 2];
            }
        }
        return dp[n];
    }
    bool isValid(int a, int b) {
        if (a == 1) return true;
        if (a == 2 && b >= 0 && b <= 5) return true;
        return false;
    }
};
```