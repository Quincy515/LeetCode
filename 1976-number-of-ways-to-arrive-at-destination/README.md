# [1976.到达目的地的方案数](https://leetcode.cn/problems/number-of-ways-to-arrive-at-destination/description/)

你在一个城市里，城市由 `n` 个路口组成，路口编号为 `0` 到 `n - 1` ，某些路口之间有 **双向** 道路。输入保证你可以从任意路口出发到达其他任意路口，且任意两个路口之间最多有一条路。

给你一个整数 `n` 和二维整数数组 `roads` ，其中 `roads[i] = [ui, vi, timei]` 表示在路口 `ui` 和 `vi` 之间有一条需要花费 `timei` 时间才能通过的道路。你想知道花费 **最少时间** 从路口 `0` 出发到达路口 `n - 1` 的方案数。

请返回花费 **最少时间** 到达目的地的 **路径数目** 。由于答案可能很大，将结果对 `109 + 7` **取余** 后返回。

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2021/07/17/graph2.png)

```
输入：n = 7, roads = [[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]
输出：4
解释：从路口 0 出发到路口 6 花费的最少时间是 7 分钟。
四条花费 7 分钟的路径分别为：
- 0 ➝ 6
- 0 ➝ 4 ➝ 6
- 0 ➝ 1 ➝ 2 ➝ 5 ➝ 6
- 0 ➝ 1 ➝ 3 ➝ 5 ➝ 6
```

**示例 2：**

```
输入：n = 2, roads = [[1,0,10]]
输出：1
解释：只有一条从路口 0 到路口 1 的路，花费 10 分钟。
```

 

**提示：**

- `1 <= n <= 200`
- `n - 1 <= roads.length <= n * (n - 1) / 2`
- `roads[i].length == 3`
- `0 <= ui, vi <= n - 1`
- `1 <= timei <= 109`
- `ui != vi`
- 任意两个路口之间至多有一条路。
- 从任意路口出发，你能够到达其他任意路口。

------

[Discussion](https://leetcode.cn/problems/number-of-ways-to-arrive-at-destination/comments/) | [Solution](https://leetcode.cn/problems/number-of-ways-to-arrive-at-destination/solution/)

**思路**

Djikstra

**题解**

```rust
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        const MOD: i64 = 1e9 as i64 + 7;
        let _len_r: usize = roads.len();
        // to build the graph
        let graph: Vec<Vec<(usize, i64)>> = {
            let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n as usize];
            for road in roads {
                let u: usize = road[0] as usize;
                let v: usize = road[1] as usize;
                let time: i64 = road[2] as i64;
                graph[u].push((v, time));
                graph[v].push((u, time));
            }
            graph
        };
        // to apply Djikstra's Algorithm
        let mut times: Vec<i64> = vec![i64::MAX; n as usize];
        times[0] = 0;
        let mut ways: Vec<i64> = vec![0; n as usize];
        ways[0] = 1;
        // used as a min heap
        let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
        heap.push((0, 0));
        while !heap.is_empty() {
            if let Some((time_u, u)) = heap.pop() {
                let time_u = -time_u;
                if time_u > times[u] {
                    continue;
                }
                for &(v, time_v) in graph[u].iter() {
                    if times[v] > time_u + time_v {
                        times[v] = time_u + time_v;
                        ways[v] = ways[u];
                        heap.push((-times[v], v));
                    } else if times[v] == time_u + time_v {
                        ways[v] = (ways[v] + ways[u]) % MOD;
                    }
                }
            }
        }
        ways[n as usize - 1] as i32
    }
}
```

