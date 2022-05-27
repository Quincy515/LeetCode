# [1791.找出星型图的中心节点](https://leetcode.cn/problems/find-center-of-star-graph/description/)

有一个无向的 **星型** 图，由 `n` 个编号从 `1` 到 `n` 的节点组成。星型图有一个 **中心** 节点，并且恰有 `n - 1` 条边将中心节点与其他每个节点连接起来。

给你一个二维整数数组 `edges` ，其中 `edges[i] = [ui, vi]` 表示在节点 `ui` 和 `vi` 之间存在一条边。请你找出并返回 `edges` 所表示星型图的中心节点。

 

**示例 1：**

![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/03/14/star_graph.png)

```
输入：edges = [[1,2],[2,3],[4,2]]
输出：2
解释：如上图所示，节点 2 与其他每个节点都相连，所以节点 2 是中心节点。
```

**示例 2：**

```
输入：edges = [[1,2],[5,1],[1,3],[1,4]]
输出：1
```

 

**提示：**

- `3 <= n <= 105`
- `edges.length == n - 1`
- `edges[i].length == 2`
- `1 <= ui, vi <= n`
- `ui != vi`
- 题目数据给出的 `edges` 表示一个有效的星型图

------

[Discussion](https://leetcode.cn/problems/find-center-of-star-graph/comments/) | [Solution](https://leetcode.cn/problems/find-center-of-star-graph/solution/)

**思路**

1、中间节点一定和所有节点连边，所以直接找两个边 (a, b) 和 (c, d);

2、当 a 等于 c 或 d 时，a 就是中间节点；否则就是 b

**题解**

```rust
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let (a, b) = (edges[0][0], edges[0][1]);
        let (c, d) = (edges[1][0], edges[1][1]);
        if a == c || a == d {
            return a;
        }
        b
    }
}
```

