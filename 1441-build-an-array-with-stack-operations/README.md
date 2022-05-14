# [1441.用栈操作构建数组](https://leetcode.cn/problems/build-an-array-with-stack-operations/description/)

给你一个目标数组 `target` 和一个整数 `n`。每次迭代，需要从 `list = {1,2,3..., n}` 中依序读取一个数字。

请使用下述操作来构建目标数组 `target` ：

- **Push**：从 `list` 中读取一个新元素， 并将其推入数组中。
- **Pop**：删除数组中的最后一个元素。
- 如果目标数组构建完成，就停止读取更多元素。

题目数据保证目标数组严格递增，并且只包含 `1` 到 `n` 之间的数字。

请返回构建目标数组所用的操作序列。

题目数据保证答案是唯一的。

 

**示例 1：**

```
输入：target = [1,3], n = 3
输出：["Push","Push","Pop","Push"]
解释： 
读取 1 并自动推入数组 -> [1]
读取 2 并自动推入数组，然后删除它 -> [1]
读取 3 并自动推入数组 -> [1,3]
```

**示例 2：**

```
输入：target = [1,2,3], n = 3
输出：["Push","Push","Push"]
```

**示例 3：**

```
输入：target = [1,2], n = 4
输出：["Push","Push"]
解释：只需要读取前 2 个数字就可以停止。
```

 

**提示：**

- `1 <= target.length <= 100`
- `1 <= target[i] <= 100`
- `1 <= n <= 100`
- `target` 是严格递增的

------

[Discussion](https://leetcode.cn/problems/build-an-array-with-stack-operations/comments/) | [Solution](https://leetcode.cn/problems/build-an-array-with-stack-operations/solution/)

**思路**

1、首先，根据题目要求，给定的数组一定是单调递增的

2、每个数都有 **入栈** 操作，如果该数不在 target 中，则加上一个 **出栈** 操作，考虑下边界情况。

**题解**

```rust
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut index = 0;
        for i in 1..n+1 {
            if target[index] == i {
                result.push("Push".to_string());
                index += 1;
                if index >= target.len() {
                    break;
                }
            } else {
                result.push("Push".to_string());
                result.push("Pop".to_string());
            }
        }
        result
    }
}
```

