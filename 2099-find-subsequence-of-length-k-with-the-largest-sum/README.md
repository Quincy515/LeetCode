# [2099.找到和最大的长度为 K 的子序列](https://leetcode.cn/problems/find-subsequence-of-length-k-with-the-largest-sum/description/)

给你一个整数数组 `nums` 和一个整数 `k` 。你需要找到 `nums` 中长度为 `k` 的 **子序列** ，且这个子序列的 **和最大** 。

请你返回 **任意** 一个长度为 `k` 的整数子序列。

**子序列** 定义为从一个数组里删除一些元素后，不改变剩下元素的顺序得到的数组。

 

**示例 1：**

```
输入：nums = [2,1,3,3], k = 2
输出：[3,3]
解释：
子序列有最大和：3 + 3 = 6 。
```

**示例 2：**

```
输入：nums = [-1,-2,3,4], k = 3
输出：[-1,3,4]
解释：
子序列有最大和：-1 + 3 + 4 = 6 。
```

**示例 3：**

```
输入：nums = [3,4,3,3], k = 2
输出：[3,4]
解释：
子序列有最大和：3 + 4 = 7 。
另一个可行的子序列为 [4, 3] 。
```

 

**提示：**

- `1 <= nums.length <= 1000`
- `-105 <= nums[i] <= 105`
- `1 <= k <= nums.length`

------

[Discussion](https://leetcode.cn/problems/find-subsequence-of-length-k-with-the-largest-sum/comments/) | [Solution](https://leetcode.cn/problems/find-subsequence-of-length-k-with-the-largest-sum/solution/)

**思路**

1、这题用到的知识点主要有以下：

- 结构体
- 重载小于号
- 优先队列
- 排序

2、将所有元素打包成（下标、值）的二元组，重载小于号，放入优先队列（大顶堆）

3、不断弹出堆顶元素，总计弹出 **k** 个

4、对这 **k** 个元素按照下标递增排序，输出它们的值

**题解**

```rust
#[derive(Eq, PartialEq)]
struct Pair {
    index: usize,
    value: i32,
}

impl std::cmp::Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value
            .cmp(&other.value)
            .then_with(|| other.index.cmp(&self.index))
    }
}
impl std::cmp::PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut heap = std::collections::BinaryHeap::new();
        nums.into_iter().enumerate().for_each(|(i, num)| {
            heap.push(Pair {
                index: i,
                value: num,
            })
        });
        let mut vec_heap = vec![];
        while k > 0 {
            vec_heap.push(heap.pop().unwrap());
            k -= 1;
        }
        vec_heap.sort_unstable_by_key(|item| item.index);
        vec_heap.into_iter().map(|x| x.value).collect()
    }
}
```

两次排序

```rust
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        nums.into_iter()
            .enumerate()
            .for_each(|(i, num)| result.push((i, num)));
        result.sort_unstable_by(|x, y| y.1.cmp(&x.1));
        result.truncate(k as usize);
        result.sort_unstable_by_key(|item| item.0);
        result.into_iter().map(|x| x.1).collect()
    }
}
```

