# [2144.打折购买糖果的最小开销](https://leetcode-cn.com/problems/minimum-cost-of-buying-candies-with-discount/description/)



一家商店正在打折销售糖果。每购买 **两个** 糖果，商店会 **免费** 送一个糖果。

免费送的糖果唯一的限制是：它的价格需要小于等于购买的两个糖果价格的 **较小值** 。

- 比方说，总共有 `4` 个糖果，价格分别为 `1` ，`2` ，`3` 和 `4` ，一位顾客买了价格为 `2` 和 `3` 的糖果，那么他可以免费获得价格为 `1` 的糖果，但不能获得价格为 `4` 的糖果。

给你一个下标从 **0** 开始的整数数组 `cost` ，其中 `cost[i]` 表示第 `i` 个糖果的价格，请你返回获得 **所有** 糖果的 **最小** 总开销。

 

**示例 1：**

```
输入：cost = [1,2,3]
输出：5
解释：我们购买价格为 2 和 3 的糖果，然后免费获得价格为 1 的糖果。
总开销为 2 + 3 = 5 。这是开销最小的 唯一 方案。
注意，我们不能购买价格为 1 和 3 的糖果，并免费获得价格为 2 的糖果。
这是因为免费糖果的价格必须小于等于购买的 2 个糖果价格的较小值。
```

**示例 2：**

```
输入：cost = [6,5,7,9,2,2]
输出：23
解释：最小总开销购买糖果方案为：
- 购买价格为 9 和 7 的糖果
- 免费获得价格为 6 的糖果
- 购买价格为 5 和 2 的糖果
- 免费获得价格为 2 的最后一个糖果
因此，最小总开销为 9 + 7 + 5 + 2 = 23 。
```

**示例 3：**

```
输入：cost = [5,5]
输出：10
解释：由于只有 2 个糖果，我们需要将它们都购买，而且没有免费糖果。
所以总最小开销为 5 + 5 = 10 。
```

 

**提示：**

- `1 <= cost.length <= 100`
- `1 <= cost[i] <= 100`

------

[Discussion](https://leetcode-cn.com/problems/minimum-cost-of-buying-candies-with-discount/comments/) | [Solution](https://leetcode-cn.com/problems/minimum-cost-of-buying-candies-with-discount/solution/)

**原理**

1、因为要购买所有的糖果，还有一些送的，所以能够让送的糖果的价值尽量大，那么我们购买的价值就会相应的小

2、但是想要得到购买的糖果，必须先买两个糖果，并且满足 它的价格 <= 购买的两个糖果价格的 较小值

3、所以，我们可以将糖果进行排列，模拟购买的过程

4、每次购买价格最大的两个糖果，让送价格第三大的糖果。每次消去3个糖果，最后如果只剩下1个则直接购买

**题解**

```rust
impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort();
        let n = cost.len();
        if n == 1 {
            return cost[0];
        }

        let mut result = 0;

        for i in (0..n).rev().step_by(3) {
            if i < 1 {
                result += cost[i];
            } else {
                result += cost[i] + cost[i - 1];
            }
        }
        result
    }
}
```

