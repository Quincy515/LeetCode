# [911.在线选举](https://leetcode-cn.com/problems/online-election/description/)

给你两个整数数组 `persons` 和 `times` 。在选举中，第 `i` 张票是在时刻为 `times[i]` 时投给候选人 `persons[i]` 的。

对于发生在时刻 `t` 的每个查询，需要找出在 `t` 时刻在选举中领先的候选人的编号。

在 `t` 时刻投出的选票也将被计入我们的查询之中。在平局的情况下，最近获得投票的候选人将会获胜。

实现 `TopVotedCandidate` 类：

- `TopVotedCandidate(int[] persons, int[] times)` 使用 `persons` 和 `times` 数组初始化对象。
- `int q(int t)` 根据前面描述的规则，返回在时刻 `t` 在选举中领先的候选人的编号。

 

**示例：**

```
输入：
["TopVotedCandidate", "q", "q", "q", "q", "q", "q"]
[[[0, 1, 1, 0, 0, 1, 0], [0, 5, 10, 15, 20, 25, 30]], [3], [12], [25], [15], [24], [8]]
输出：
[null, 0, 1, 1, 0, 0, 1]

解释：
TopVotedCandidate topVotedCandidate = new TopVotedCandidate([0, 1, 1, 0, 0, 1, 0], [0, 5, 10, 15, 20, 25, 30]);
topVotedCandidate.q(3); // 返回 0 ，在时刻 3 ，票数分布为 [0] ，编号为 0 的候选人领先。
topVotedCandidate.q(12); // 返回 1 ，在时刻 12 ，票数分布为 [0,1,1] ，编号为 1 的候选人领先。
topVotedCandidate.q(25); // 返回 1 ，在时刻 25 ，票数分布为 [0,1,1,0,0,1] ，编号为 1 的候选人领先。（在平局的情况下，1 是最近获得投票的候选人）。
topVotedCandidate.q(15); // 返回 0
topVotedCandidate.q(24); // 返回 0
topVotedCandidate.q(8); // 返回 1
```

 

**提示：**

- `1 <= persons.length <= 5000`
- `times.length == persons.length`
- `0 <= persons[i] < persons.length`
- `0 <= times[i] <= 109`
- `times` 是一个严格递增的有序数组
- `times[0] <= t <= 109`
- 每个测试用例最多调用 `104` 次 `q`

------

[Discussion](https://leetcode-cn.com/problems/online-election/comments/) | [Solution](https://leetcode-cn.com/problems/online-election/solution/)

**思路**

1、预处理每个时间点的赢家

2、对于每次询问，二分查找小于等于给定时间 **t** 的最大下标，通过下标去索引赢家

**题解**

```rust
struct TopVotedCandidate {
    wins: Vec<i32>,
    times: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut hash = std::collections::HashMap::new();
        let mut top_persion = -1;
        let mut wins = Vec::new();
        for person in persons.iter() {
            let count = hash.entry(person).or_insert(0);
            *count += 1;
            if top_persion == -1 || *count >= *hash.get(&top_persion).unwrap_or(&0) {
                top_persion = *person;
            }
            wins.push(top_persion);
        }
        Self { wins, times }
    }

    fn q(&self, t: i32) -> i32 {
        let (mut left, mut right) = (0, self.times.len());
        let mut result = 0;
        while left < right {
            let mid = left + (right - left) / 2;
            if self.times[mid] <= t {
                result = mid as i32;
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        self.wins[result as usize]
    }
}
```

