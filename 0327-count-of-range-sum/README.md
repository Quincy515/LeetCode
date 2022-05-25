## [327.区间和的个数](https://leetcode.cn/problems/count-of-range-sum/description/)

给你一个整数数组 `nums` 以及两个整数 `lower` 和 `upper` 。求数组中，值位于范围 `[lower, upper]` （包含 `lower` 和 `upper`）之内的 **区间和的个数** 。

**区间和** `S(i, j)` 表示在 `nums` 中，位置从 `i` 到 `j` 的元素之和，包含 `i` 和 `j` (`i` ≤ `j`)。

 

**示例 1：**

```
输入：nums = [-2,5,-1], lower = -2, upper = 2
输出：3
解释：存在三个区间：[0,0]、[2,2] 和 [0,2] ，对应的区间和分别是：-2 、-1 、2 。
```

**示例 2：**

```
输入：nums = [0], lower = 0, upper = 0
输出：1
```

 

**提示：**

- `1 <= nums.length <= 105`
- `-231 <= nums[i] <= 231 - 1`
- `-105 <= lower <= upper <= 105`
- 题目数据保证答案是一个 **32 位** 的整数

------

[Discussion](https://leetcode.cn/problems/count-of-range-sum/comments/) | [Solution](https://leetcode.cn/problems/count-of-range-sum/solution/)

**思路**

树状数组 + 离散化

**题解**

使用 树状数组模版 解题 **TLE** 超时

```rust
struct Solution;
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut tree = FenwickTree {
            sums: vec![0; nums.len() + 1],
        };
        for (i, &num) in nums.iter().enumerate() {
            tree.update(i + 1, num);
        }

        let mut result = 0;
        let n = nums.len();
        for i in 0..n {
            for j in i..n {
                let right = tree.query(j + 1);
                let left = tree.query(i);
                result += if right - left <= upper && right - left >= lower {
                    1
                } else {
                    0
                };
            }
        }
        for i in 1..=n {
            result += tree.query(get_index())
        }

        result
    }
}

struct FenwickTree {
    sums: Vec<i32>,
}

impl FenwickTree {
    fn update(&mut self, mut i: usize, delta: i32) {
        while i < self.sums.len() {
            self.sums[i] += delta;
            i += self.lowbit(i);
        }
    }

    fn query(&self, mut i: usize) -> i32 {
        let mut res = 0;
        while i > 0 {
            res += self.sums[i];
            i -= self.lowbit(i);
        }
        res
    }

    fn lowbit(&self, x: usize) -> usize {
        x & (!x + 1)
    }
}
```

1. 遇到任何区间和的问题，我们应该首先想到前缀和，首先计算数组的前缀和，这个问题中数据范围是 32位 整型，计算前缀和必然会溢出，所以可以用另一个 64 位的数组来存储，并且数组下标从 1 开始。
2. 对于任意一个 $i$，问题转变成了求满足以下不等式的 $t(0 ≤ t < i)$ 的数量：

​														$ lower \leq sum[i] - sum[t] \leq upper$

3. 既然如此，我们可以枚举 `i`，这样一来，$sum[i]$ 就变成了常量。虽然 $sum[i]$ 变成了常量，但是 $sum[t]$ 还是变量。于是对上述不等式进行一个变换：
   														$sum[i]−upper ≤ sum[t] ≤ sum[i]−lower$

这样一来，问题就转化成了求区间 $[sum[i] - upper, sum[i] - lower]$ 中 $sum[t]$ 的个数。

4. 假设有一种容器，这种容器可以在 $O(log_2n)$ 的时间复杂度内插入数据、可以在 $O(log_2n)$  的时间复杂度内统计前缀和，这样就能轻松满足上述条件了。满足这两个条件的数据结构有两种：*树状数组* 和 *线段树*。
5. 只要把接口抽象出来，具体是用树状数组实现，还是用线段树实现，在使用的人角度可以不用关心，但是有个前提，数据范围太大了，对于树状数组而言，数据范围应该在 $[1, x]$ 之间，并且保持整数，$x$ 一般在 $10^6$ 以下；对于线段树而言，数据范围也是整数，并且保持左右端点的差值在 $10^6$ 以下。
6. 但是数据范围较大，所以需要进行离散化，将 **大的数据** 映射到 **小的下标**。
7. 需要离散化的值主要有 `pre_sums[i]` 、 `pre_sums[i] − lower` 、 `pre_sum[i]−upper`。

时间复杂度是 $O(nlog_2n)$

```rust
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // 前缀和
        let mut pre_sums = vec![0];
        let mut s: i64 = 0;
        nums.iter().for_each(|&num| {
            s += num as i64;
            pre_sums.push(s);
        });
		// 转换的公式并进行排序去重
        let values: std::collections::BTreeSet<i64> = pre_sums
            .iter()
            .flat_map(|&x| vec![x, x - lower as i64, x - upper as i64])
            .collect();
		// 将 大的数据 映射到 小的下标
        let rank: std::collections::HashMap<i64, usize> = values
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i + 1))
            .collect();

        // 树状数组
        let mut result = 0;
        let mut tree = FenwickTree {
            sums: vec![0; rank.len()+1],
        };
        for &x in &pre_sums {
            result += tree.query(rank[&(x - lower as i64)])
                - tree.query(rank[&(x - upper as i64)] - 1);
            tree.update(rank[&x], 1);
        }
        result
    }
}

// 树状数组模版
struct FenwickTree {
    sums: Vec<i32>,
}

impl FenwickTree {
    fn update(&mut self, mut i: usize, delta: i32) {
        while i < self.sums.len() {
            self.sums[i] += delta;
            i += self.lowbit(i);
        }
    }

    fn query(&self, mut i: usize) -> i32 {
        let mut res = 0;
        while i > 0 {
            res += self.sums[i];
            i -= self.lowbit(i);
        }
        res
    }

    fn lowbit(&self, x: usize) -> usize {
        x & (!x + 1)
    }
}

```

