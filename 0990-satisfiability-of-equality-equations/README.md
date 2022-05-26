## [990.等式方程的可满足性](https://leetcode.cn/problems/satisfiability-of-equality-equations/description/)

给定一个由表示变量之间关系的字符串方程组成的数组，每个字符串方程 `equations[i]` 的长度为 `4`，并采用两种不同的形式之一：`"a==b"` 或 `"a!=b"`。在这里，a 和 b 是小写字母（不一定不同），表示单字母变量名。

只有当可以将整数分配给变量名，以便满足所有给定的方程时才返回 `true`，否则返回 `false`。 

**示例 1：**

```
输入：["a==b","b!=a"]
输出：false
解释：如果我们指定，a = 1 且 b = 1，那么可以满足第一个方程，但无法满足第二个方程。没有办法分配变量同时满足这两个方程。
```

**示例 2：**

```
输入：["b==a","a==b"]
输出：true
解释：我们可以指定 a = 1 且 b = 1 以满足满足这两个方程。
```

**示例 3：**

```
输入：["a==b","b==c","a==c"]
输出：true
```

**示例 4：**

```
输入：["a==b","b!=c","c==a"]
输出：false
```

**示例 5：**

```
输入：["c==c","b==d","x!=z"]
输出：true
```

 

**提示：**

1. `1 <= equations.length <= 500`
2. `equations[i].length == 4`
3. `equations[i][0]` 和 `equations[i][3]` 是小写字母
4. `equations[i][1]` 要么是 `'='`，要么是 `'!'`
5. `equations[i][2]` 是 `'='`

------

[Discussion](https://leetcode.cn/problems/satisfiability-of-equality-equations/comments/) | [Solution](https://leetcode.cn/problems/satisfiability-of-equality-equations/solution/)

**思路**

1、首先把所有相等的情况用并查集进行合并

2、然后对所有不相等的情况进行判定是否在一个集合，如果不在则返回 false

3、判定完毕没有出现不在一个集合的，则返回 true

**题解**

```rust
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut ufs = UnionFindSet::new(26);
        for equation in &equations {
            let str = equation.to_string().into_bytes();
            if str[1] == b'=' {
                ufs.union(
                    str[0] as usize - 'a' as usize,
                    str[3] as usize - 'a' as usize,
                );
            }
        }

        for equation in equations {
            let str = equation.chars().collect::<Vec<char>>();
            if str[1] == '!'
                && ufs.find(str[0] as usize - 'a' as usize)
                    == ufs.find(str[3] as usize - 'a' as usize)
            {
                return false;
            }
        }
        true
    }
}

struct UnionFindSet {
    parent: Vec<usize>,
    rank: Vec<i32>, // rank[i] 表示以 i 为根节点为 i 的树的高度
}

impl UnionFindSet {
    fn new(n: usize) -> Self {
        Self {
            rank: vec![0; n],
            parent: (0..n).collect::<Vec<_>>(),
        }
    }
    // 合并元素 i 和元素 j 所属的集合
    fn union(&mut self, i: usize, j: usize) -> bool {
        let (i_root, j_root) = (self.find(i), self.find(j));
        if i_root == j_root {
            return false;
        }

        if self.rank[i_root] > self.rank[j_root] {
            self.parent[j_root] = i_root;
        } else if self.rank[i_root] < self.rank[j_root] {
            self.parent[i_root] = j_root;
        } else {
            self.parent[i_root] = j_root;
            self.rank[j_root] += 1;
        }

        true
    }
    // 查找元素 i 所对应的集合编号
    fn find(&mut self, i: usize) -> usize {
        if i != self.parent[i] {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }
}
```

