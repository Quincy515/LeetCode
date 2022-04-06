# 121. 买卖股票的最佳时机
给定一个数组，它的第 *i* 个元素是一支给定股票第 *i* 天的价格。

如果你最多只允许完成一笔交易（即买入和卖出一支股票），设计一个算法来计算你所能获取的最大利润。

注意你不能在买入股票前卖出股票。

## 示例 1:
<pre>
<strong>输入:</strong> [7,1,5,3,6,4]
<strong>输出:</strong> 5
<strong>解释:</strong> 在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
     注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格。
</pre>

## 示例 2:
<pre>
<strong>输入:</strong> [7,6,4,3,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 在这种情况下, 没有交易完成, 所以最大利润为 0。
</pre>

## 题解：
### Rust
```rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (n, mut max, mut cur_max) = (prices.len(), vec![0; prices.len()], 0);
        for i in (0..=n - 1).rev() {
            max[i] = cur_max;
            if prices[i] > cur_max {
                cur_max = prices[i];
            }
        }
        let mut result = 0;
        for i in 0..n {
            if result < max[i] - prices[i] {
                result = max[i] - prices[i];
            }
        }
        result
    }
}

```

### Go
```go
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (n, mut max, mut cur_max) = (prices.len(), vec![0; prices.len()], 0);
        for i in (0..=n - 1).rev() {
            max[i] = cur_max;
            if prices[i] > cur_max {
                cur_max = prices[i];
            }
        }
        let mut result = 0;
        for i in 0..n {
            if result < max[i] - prices[i] {
                result = max[i] - prices[i];
            }
        }
        result
    }
}

```

### JavaScript
```javascript
/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
        let n = prices.length
        let max = new Array(n)
        let curMax = 0
        for (let i = n-1; i >= 0; --i) {
            max[i] = curMax
            if (prices[i] > curMax) [
                curMax = prices[i]
            ]
        }
        let result = 0
        for (let i = 0; i < n; ++i) {
            if (result < max[i]-prices[i]) {
                result = max[i]-prices[i]
            }
        }
        return result
    }
```

### Python
```python
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        maxPrices = [0] * n
        curMax = 0
        for i in range(n-1,-1,-1):
            maxPrices[i] = curMax
            if prices[i] > curMax:
                curMax = prices[i]
        result = 0
        for i in range(n):
            if result < maxPrices[i] - prices[i]:
                result = maxPrices[i] - prices[i]
        return result

```

### C++
```c++
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int n = prices.size();
        vector<int> max(n);
        int curMax = 0;
        for (int i = n - 1; i >= 0; --i) {
            max[i] = curMax;
            if (prices[i] > curMax) {
                curMax = prices[i];
            }
        }
        int result = 0;
        for (int i = 0; i < n; ++i) {
            if (result < max[i] - prices[i]) {
                result = max[i] - prices[i];
            }
        }
        return result;
    }
};

```