## [1420.生成数组](https://leetcode.cn/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/description/)

给你三个整数 `n`、`m` 和 `k` 。下图描述的算法用于找出正整数数组中最大的元素。

![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/04/19/e.png)

请你生成一个具有下述属性的数组 `arr` ：

- `arr` 中有 `n` 个整数。
- `1 <= arr[i] <= m` 其中 `(0 <= i < n)` 。
- 将上面提到的算法应用于 `arr` ，`search_cost` 的值等于 `k` 。

返回上述条件下生成数组 `arr` 的 **方法数** ，由于答案可能会很大，所以 **必须** 对 `10^9 + 7` 取余。

 

**示例 1：**

```
输入：n = 2, m = 3, k = 1
输出：6
解释：可能的数组分别为 [1, 1], [2, 1], [2, 2], [3, 1], [3, 2] [3, 3]
```

**示例 2：**

```
输入：n = 5, m = 2, k = 3
输出：0
解释：没有数组可以满足上述条件
```

**示例 3：**

```
输入：n = 9, m = 1, k = 1
输出：1
解释：可能的数组只有 [1, 1, 1, 1, 1, 1, 1, 1, 1]
```

**示例 4：**

```
输入：n = 50, m = 100, k = 25
输出：34549172
解释：不要忘了对 1000000007 取余
```

**示例 5：**

```
输入：n = 37, m = 17, k = 7
输出：418930126
```

 

**提示：**

- `1 <= n <= 50`
- `1 <= m <= 100`
- `0 <= k <= n`

------

[Discussion](https://leetcode.cn/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/comments/) | [Solution](https://leetcode.cn/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/solution/)

**思路**

1、首先，我们需要知道什么情况下，search_cost 会进行累加

2、设想有一个单调栈，栈中元素从 **栈底** 到 **栈顶** 都是单调递增的，那么，从左往右扫描数据，遇到红色的点就应该入栈，并且入栈的次数应该和 `search_cost` 相等

3、注意，这里需要取模，但是直接取模效率较低，可以采用减法代替取模

**1. 设计状态**

于是，就可以设计状态如下：$dp[i][j][t]$ 表示总共有 $i$ 个数，单调栈中栈顶元素为 $j$，且单调栈的总元素个数为 $t$ 个。

**2. 最终状态**

当总共有 $n$ 个数，最大的数为 $m$，且 `search_cost` 为 $k$ 时，我们要求的方案数就应该是 $\sum_{j=1}^{m} dp[n][j][k]$。

**3. 初始状态**

当只有一个数的时候，单调栈中元素的个数必定是 1 个，它就是初始状态，即：$dp[1][j][1] = 1(1\le j \le m)$。

**4. 状态转移**

当 $dp[i][j][t]$ 已知，也就是前 $i$ 个数中，单调栈中元素个数为 $t$ 个，且栈顶元素的值为 $j$ 的时候的方案数为 $dp[i][j][t]$。

那么，我们可以往后面继续塞入一个数，塞入的数可以是 $jj(1 \le jj \le m)$，分两种情况讨论：

- $j \ge jj$ ，那么引入 $jj$ 并不会对单调栈产生影响，状态转移到了 $dp[i+1][j][t]$；
- $j \le jj$，那么引入 $jj$ 就会将 $jj$ 插入到单调栈中，使得栈中的元素增加了一个，状态转移到了 $dp[i+1][jj][t+1]$；

**时间复杂度**

状态数 $O(nmk)$，状态转移 $O(m)$，最坏时间复杂度 $O(nm^2k)$。

> 动态规则的求解过程比较单一，可以先设计状态，再考虑最终状态，边界状态，再进行状态转移。

**题解**

```rust
impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mood = 1000000007;
        let (n, m, k) = (n as usize, m as usize, k as usize);
        let mut dp = vec![vec![vec![0; 55]; 105]; 105];
        for j in 1..m + 1 {
            dp[1][j][1] = 1;
        }
        for i in 1..n + 1 {
            for j in 1..m + 1 {
                for t in 1..k + 1 {
                    for _ in 1..j + 1 {
                        dp[i + 1][j][t] += dp[i][j][t];
                        dp[i + 1][j][t] %= mood;
                    }
                    for jj in j + 1..m + 1 {
                        dp[i + 1][jj][t + 1] += dp[i][j][t];
                        dp[i + 1][jj][t + 1] %= mood;
                    }
                }
            }
        }

        let mut result = 0;
        for i in 1..m + 1 {
            result += dp[n][i][k];
            result %= mood;
        }
        result
    }
}
```

