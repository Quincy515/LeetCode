# [207.课程表](https://leetcode.cn/problems/course-schedule/description/)

你这个学期必须选修 `numCourses` 门课程，记为 `0` 到 `numCourses - 1` 。

在选修某些课程之前需要一些先修课程。 先修课程按数组 `prerequisites` 给出，其中 `prerequisites[i] = [ai, bi]` ，表示如果要学习课程 `ai` 则 **必须** 先学习课程 `bi` 。

- 例如，先修课程对 `[0, 1]` 表示：想要学习课程 `0` ，你需要先完成课程 `1` 。

请你判断是否可能完成所有课程的学习？如果可以，返回 `true` ；否则，返回 `false` 。

 

**示例 1：**

```
输入：numCourses = 2, prerequisites = [[1,0]]
输出：true
解释：总共有 2 门课程。学习课程 1 之前，你需要完成课程 0 。这是可能的。
```

**示例 2：**

```
输入：numCourses = 2, prerequisites = [[1,0],[0,1]]
输出：false
解释：总共有 2 门课程。学习课程 1 之前，你需要先完成课程 0 ；并且学习课程 0 之前，你还应先完成课程 1 。这是不可能的。
```

 

**提示：**

- `1 <= numCourses <= 105`
- `0 <= prerequisites.length <= 5000`
- `prerequisites[i].length == 2`
- `0 <= ai, bi < numCourses`
- `prerequisites[i]` 中的所有课程对 **互不相同**

------

[Discussion](https://leetcode.cn/problems/course-schedule/comments/) | [Solution](https://leetcode.cn/problems/course-schedule/solution/)

**题解**

```rust
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adjs = vec![vec![]; num_courses as usize];
        let mut indegrees = vec![0; num_courses as usize];
        for i in 0..prerequisites.len() {
            adjs[prerequisites[i][1] as usize].push(prerequisites[i][0]);
            indegrees[prerequisites[i][0] as usize] += 1;
        }
        let mut zero_in_degrees = vec![];
        for (i, _) in indegrees.iter().enumerate() {
            if indegrees[i] == 0 {
                zero_in_degrees.push(i);
            }
        }
        let mut zero_in_degrees_count = 0;
        while !zero_in_degrees.is_empty() {
            let coursei = zero_in_degrees.pop().unwrap();
            zero_in_degrees_count += 1;
            for coursej in &adjs[coursei] {
                indegrees[*coursej as usize] -= 1;
                if indegrees[*coursej as usize] == 0 {
                    zero_in_degrees.push(*coursej as usize);
                }
            }
        }

        zero_in_degrees_count == num_courses
    }
}
```

