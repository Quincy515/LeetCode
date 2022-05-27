# [797.所有可能的路径](https://leetcode.cn/problems/all-paths-from-source-to-target/description/)

给你一个有 `n` 个节点的 **有向无环图（DAG）**，请你找出所有从节点 `0` 到节点 `n-1` 的路径并输出（**不要求按特定顺序**）

 `graph[i]` 是一个从节点 `i` 可以访问的所有节点的列表（即从节点 `i` 到节点 `graph[i][j]`存在一条有向边）。

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2020/09/28/all_1.jpg)

```
输入：graph = [[1,2],[3],[3],[]]
输出：[[0,1,3],[0,2,3]]
解释：有两条路径 0 -> 1 -> 3 和 0 -> 2 -> 3
```

**示例 2：**

![img](https://assets.leetcode.com/uploads/2020/09/28/all_2.jpg)

```
输入：graph = [[4,3,1],[3,2,4],[3],[4],[]]
输出：[[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]
```

 

**提示：**

- `n == graph.length`
- `2 <= n <= 15`
- `0 <= graph[i][j] < n`
- `graph[i][j] != i`（即不存在自环）
- `graph[i]` 中的所有元素 **互不相同**
- 保证输入为 **有向无环图（DAG）**

 

------

[Discussion](https://leetcode.cn/problems/all-paths-from-source-to-target/comments/) | [Solution](https://leetcode.cn/problems/all-paths-from-source-to-target/solution/)

**思路**

1、求所有可能的路径，一定是暴力搜索所有情况，直接采用 dfs 从 0 出发，到 n-1 结束，注意需要回溯

**题解**

```rust
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::dfs(&graph, 0, &mut vec![0], &mut result);
        result
    }

    fn dfs(graph: &[Vec<i32>], now: usize, stack: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if now == graph.len() - 1 {
            result.push(stack.clone());
            return;
        }

        for i in 0..graph[now].len() {
            stack.push(graph[now][i]);
            Self::dfs(graph, graph[now][i] as usize, stack, result);
            stack.pop();
        }
    }
}
```

