# [1052.爱生气的书店老板](https://leetcode-cn.com/problems/grumpy-bookstore-owner/description/)

有一个书店老板，他的书店开了 `n` 分钟。每分钟都有一些顾客进入这家商店。给定一个长度为 `n` 的整数数组 `customers` ，其中 `customers[i]` 是在第 `i` 分钟开始时进入商店的顾客的编号，所有这些顾客在第 `i` 分钟结束后离开。

在某些时候，书店老板会生气。 如果书店老板在第 `i` 分钟生气，那么 `grumpy[i] = 1`，否则 `grumpy[i] = 0`。

当书店老板生气时，那一分钟的顾客就会不满意，若老板不生气则顾客是满意的。

书店老板知道一个秘密技巧，能抑制自己的情绪，可以让自己连续 `minutes` 分钟不生气，但却只能使用一次。

请你返回 *这一天营业下来，最多有多少客户能够感到满意* 。
 

**示例 1：**

```
输入：customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], minutes = 3
输出：16
解释：书店老板在最后 3 分钟保持冷静。
感到满意的最大客户数量 = 1 + 1 + 1 + 1 + 7 + 5 = 16.
```

**示例 2：**

```
输入：customers = [1], grumpy = [0], minutes = 1
输出：1
```

 

**提示：**

- `n == customers.length == grumpy.length`
- `1 <= minutes <= n <= 2 * 104`
- `0 <= customers[i] <= 1000`
- `grumpy[i] == 0 or 1`

------

[Discussion](https://leetcode-cn.com/problems/grumpy-bookstore-owner/comments/) | [Solution](https://leetcode-cn.com/problems/grumpy-bookstore-owner/solution/)

**思路**

1、记录两个前缀和，一个表示不管不生气的前缀和，一个表示考虑了生气的情况下的前缀和

2、定义一个滑动窗口进行滑动操作，当窗口正好为 **k** 时，则进行判定；令考虑生气的情况下的人数为 **x**，当前窗口 **[i, j]** 的人数，考虑生气的情况下为 **y**，不考虑生气的情况下的人数为 **z**，结果就是 **x - y + z**，计算最大值。其中 **y** 和 **z** 可以通过前缀和差分求得

**题解**

```rust
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let max_n = customers.len().max(grumpy.len()) + 1;
        let (mut cust_sum, mut cust_gru) = (vec![0; max_n], vec![0; max_n]);
        let n = customers.len();
        for i in 1..=n {
            let index = i - 1;
            cust_sum[i] = cust_sum[i - 1] + customers[index];
            cust_gru[i] = cust_gru[i - 1] + (1 - grumpy[index]) * customers[index]
        }
        println!("cust_sum: {:?}, cust_gru: {:?}", cust_sum, cust_gru);
        let (mut i, mut j, mut result) = (1, 0, 0);
        while j < n {
            j += 1;
            while j - i + 1 > minutes as usize {
                i += 1;
            }
            if j - i + 1 == minutes as usize {
                let val =
                    cust_gru[n] - (cust_gru[j] - cust_gru[i - 1]) + cust_sum[j] - cust_sum[i - 1];
                result = result.max(val);
            }
        }
        result
    }
}
```

