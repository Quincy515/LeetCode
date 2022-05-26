## [684.冗余连接](https://leetcode.cn/problems/redundant-connection/description/)

树可以看成是一个连通且 **无环** 的 **无向** 图。

给定往一棵 `n` 个节点 (节点值 `1～n`) 的树中添加一条边后的图。添加的边的两个顶点包含在 `1` 到 `n` 中间，且这条附加的边不属于树中已存在的边。图的信息记录于长度为 `n` 的二维数组 `edges` ，`edges[i] = [ai, bi]` 表示图中在 `ai` 和 `bi` 之间存在一条边。

请找出一条可以删去的边，删除后可使得剩余部分是一个有着 `n` 个节点的树。如果有多个答案，则返回数组 `edges` 中最后出现的边。

**示例 1：**

![img](https://pic.leetcode-cn.com/1626676174-hOEVUL-image.png)

```
输入: edges = [[1,2], [1,3], [2,3]]
输出: [2,3]
```

**示例 2：**

![img](https://pic.leetcode-cn.com/1626676179-kGxcmu-image.png)

```
输入: edges = [[1,2], [2,3], [3,4], [1,4], [1,5]]
输出: [1,4] 
```

**提示:**

- `n == edges.length`
- `3 <= n <= 1000`
- `edges[i].length == 2`
- `1 <= ai < bi <= edges.length`
- `ai != bi`
- `edges` 中无重复元素
- 给定的图是连通的 

------

[Discussion](https://leetcode.cn/problems/redundant-connection/comments/) | [Solution](https://leetcode.cn/problems/redundant-connection/solution/)

**思考**

并查集模版

Union find set 是根据对象两两之间的直接关系来快速查询任意两个对象之间时候存在直接或间接的关系。

从本质上讲，并查集是一组集合，存在直接或间接关系的对象放到一个集合中，不存在任何关系的对象被分隔到不同的集合中。在这组集合上，有两个主要的操作：“并”（union）和 ”查“（find）。

- 查：两个对象是否属于同一个集合。
- 并：是将两个集合合并在一起。

**题解**

```rust
struct UnionFindSet {
    parents: Vec<usize>,
    ranks: Vec<i32>,
}

impl UnionFindSet {
    fn new(n: usize) -> Self {
        Self {
            ranks: vec![0; n + 1],
            parents: (0..n + 1).collect::<Vec<_>>(),
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let (root_i, root_j) = (self.find(i), self.find(j));
        if root_i == root_j {
            return false;
        }

        if self.ranks[root_i] > self.ranks[root_j] {
            self.parents[root_j] = root_i;
        } else if self.ranks[root_i] < self.ranks[root_j] {
            self.parents[root_i] = root_j;
        } else {
            self.parents[root_i] = root_j;
            self.ranks[root_j] += 1;
        }

        true
    }

    fn find(&mut self, i: usize) -> usize {
        if i != self.parents[i] {
            self.parents[i] = self.find(self.parents[i]);
        }
        self.parents[i]
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ufs = UnionFindSet::new(edges.len());
        for edge in edges {
            if !ufs.union(edge[0] as usize, edge[1] as usize) {
                return edge;
            }
        }
        vec![]
    }
}
```



