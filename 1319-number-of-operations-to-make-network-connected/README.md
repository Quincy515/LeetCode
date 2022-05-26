## [1319.连通网络的操作次数](https://leetcode.cn/problems/number-of-operations-to-make-network-connected/description/)

用以太网线缆将 `n` 台计算机连接成一个网络，计算机的编号从 `0` 到 `n-1`。线缆用 `connections` 表示，其中 `connections[i] = [a, b]` 连接了计算机 `a` 和 `b`。

网络中的任何一台计算机都可以通过网络直接或者间接访问同一个网络中其他任意一台计算机。

给你这个计算机网络的初始布线 `connections`，你可以拔开任意两台直连计算机之间的线缆，并用它连接一对未直连的计算机。请你计算并返回使所有计算机都连通所需的最少操作次数。如果不可能，则返回 -1 。 

 

**示例 1：**

**![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/11/sample_1_1677.png)**

```
输入：n = 4, connections = [[0,1],[0,2],[1,2]]
输出：1
解释：拔下计算机 1 和 2 之间的线缆，并将它插到计算机 1 和 3 上。
```

**示例 2：**

**![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/11/sample_2_1677.png)**

```
输入：n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
输出：2
```

**示例 3：**

```
输入：n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
输出：-1
解释：线缆数量不足。
```

**示例 4：**

```
输入：n = 5, connections = [[0,1],[0,2],[3,4],[2,3]]
输出：0
```

 

**提示：**

- `1 <= n <= 10^5`
- `1 <= connections.length <= min(n*(n-1)/2, 10^5)`
- `connections[i].length == 2`
- `0 <= connections[i][0], connections[i][1] < n`
- `connections[i][0] != connections[i][1]`
- 没有重复的连接。
- 两台计算机不会通过多条线缆连接。

------

[Discussion](https://leetcode.cn/problems/number-of-operations-to-make-network-connected/comments/) | [Solution](https://leetcode.cn/problems/number-of-operations-to-make-network-connected/solution/)

**思路**

1、首先对每条边进行并查集的合并操作，能够合并则合并；不能合并则计数器 edge 自增

2、然后利用并查集判断连通块的数目 count

3、于是，需要 count-1 条边才能把他们连接起来，所以如果 edge >= count - 1 则输出 count - 1 否则输出 -1

**题解**

```rust
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let (mut edge, mut count) = (0, 0);
        let mut hash = std::collections::HashMap::new();
        let mut uf = UnionFindSet::new(n as usize);
        for connection in connections.iter() {
            let (a, b) = (connection[0], connection[1]);
            if !uf.union(a as usize, b as usize) {
                edge += 1;
            }
        }

        for i in 0..n {
            let set_id = uf.find(i as usize);
            if !hash.contains_key(&set_id) {
                hash.insert(set_id, 1);
                count += 1;
            }
        }

        if edge >= count - 1 {
            return count - 1;
        }
        -1
    }
}

struct UnionFindSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFindSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect::<Vec<_>>(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let (i_root, j_root) = (self.find(i), self.find(j));
        if i_root == j_root {
            return false;
        }
        if self.rank[i_root] < self.rank[j_root] {
            self.parent[i_root] = j_root;
        } else if self.rank[i_root] > self.rank[j_root] {
            self.parent[j_root] = i_root;
        } else {
            self.parent[j_root] = i_root;
            self.rank[i_root] += 1;
        }
        true
    }
}
```

