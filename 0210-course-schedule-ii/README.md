# [210.课程表 II](https://leetcode.cn/problems/course-schedule-ii/description/)

现在你总共有 `numCourses` 门课需要选，记为 `0` 到 `numCourses - 1`。给你一个数组 `prerequisites` ，其中 `prerequisites[i] = [ai, bi]` ，表示在选修课程 `ai` 前 **必须** 先选修 `bi` 。

- 例如，想要学习课程 `0` ，你需要先完成课程 `1` ，我们用一个匹配来表示：`[0,1]` 。

返回你为了学完所有课程所安排的学习顺序。可能会有多个正确的顺序，你只要返回 **任意一种** 就可以了。如果不可能完成所有课程，返回 **一个空数组** 。

 

**示例 1：**

```
输入：numCourses = 2, prerequisites = [[1,0]]
输出：[0,1]
解释：总共有 2 门课程。要学习课程 1，你需要先完成课程 0。因此，正确的课程顺序为 [0,1] 。
```

**示例 2：**

```
输入：numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
输出：[0,2,1,3]
解释：总共有 4 门课程。要学习课程 3，你应该先完成课程 1 和课程 2。并且课程 1 和课程 2 都应该排在课程 0 之后。
因此，一个正确的课程顺序是 [0,1,2,3] 。另一个正确的排序是 [0,2,1,3] 。
```

**示例 3：**

```
输入：numCourses = 1, prerequisites = []
输出：[0]
```

 

**提示：**

- `1 <= numCourses <= 2000`
- `0 <= prerequisites.length <= numCourses * (numCourses - 1)`
- `prerequisites[i].length == 2`
- `0 <= ai, bi < numCourses`
- `ai != bi`
- 所有`[ai, bi]` **互不相同**

------

[Discussion](https://leetcode.cn/problems/course-schedule-ii/comments/) | [Solution](https://leetcode.cn/problems/course-schedule-ii/solution/) ｜hepta

**思路**

[labuladong 的算法小抄 > 第一章、手把手刷数据结构 > 手把手刷图算法 > 拓扑排序详解及运用](https://labuladong.github.io/algo/2/20/48/)

这道题就是上道题 [LeetCode 207. 课程表](https://leetcode.cn/problems/course-schedule/solution/custer-xue-xi-bi-ji-jian-ce-by-custerfun-1rxh/) 的进阶版，不是仅仅让你判断是否可以完成所有课程，而是进一步让你返回一个合理的上课顺序，保证开始修每个课程时，前置的课程都已经修完。

![image.png](https://pic.leetcode-cn.com/1654325690-TfYgLy-image.png)

直观地说就是，让你把一幅图「拉平」，而且这个「拉平」的图里面，所有箭头方向都是一致的，比如上图所有箭头都是朝右的。

很显然，如果一幅有向图中存在环，是无法进行拓扑排序的，因为肯定做不到所有箭头方向一致；反过来，如果一幅图是「有向无环图」，那么一定可以进行拓扑排序。

**题解**

```rust
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let graph = build_graph(num_courses, prerequisites);
        let mut cycle = false;
        let mut postorder = vec![];
        let (mut path, mut visited) = (vec![false; n], vec![false; n]);
        // 遍历图
        for i in 0..num_courses {
            traverse(
                &graph,
                i as usize,
                &mut path,
                &mut visited,
                &mut postorder,
                &mut cycle,
            )
        }

        // 有环图无法进行拓扑排序
        if cycle {
            return vec![];
        }

        // 逆后序遍历结果即为拓扑排序结果
        postorder.into_iter().rev().collect()
    }
}

// 使用邻接表构建图
fn build_graph(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut graph = vec![Vec::new(); num_courses as usize];
    for edge in prerequisites.iter() {
        let (from, to) = (edge[1] as usize, edge[0] as usize);
        graph[from].push(to);
    }
    graph
}

// 遍历图
fn traverse(
    graph: &[Vec<usize>],
    v: usize,
    path: &mut Vec<bool>,
    visited: &mut Vec<bool>,
    postorder: &mut Vec<i32>,
    cycle: &mut bool,
) {
    if path[v] {
        *cycle = true;
    }
    if visited[v] || *cycle {
        return;
    }
    visited[v] = true;
    path[v] = true;
    for &w in graph[v].iter() {
        traverse(graph, w, path, visited, postorder, cycle);
    }
    postorder.push(v as i32);
    path[v] = false;
}
```

