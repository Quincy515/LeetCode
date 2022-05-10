<p>给定两个整数 <code>n</code> 和 <code>k</code>，返回范围 <code>[1, n]</code> 中所有可能的 <code>k</code> 个数的组合。</p>

<p>你可以按 <strong>任何顺序</strong> 返回答案。</p>

<p> </p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>n = 4, k = 2
<strong>输出：</strong>
[
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4],
]</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>n = 1, k = 1
<strong>输出：</strong>[[1]]</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= n <= 20</code></li>
	<li><code>1 <= k <= n</code></li>
</ul>
<div><div>Related Topics</div><div><li>数组</li><li>回溯</li></div></div><br>

## 题解
### Rust
```rust
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];

        Self::backtrack(n, k, 1, &mut vec![], &mut result);

        result
    }

    // n, k 必须的参数
    // step 阶段
    // path 路径
    // step 选与不选 - 可选列表
    fn backtrack(n: i32, k: i32, step: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if path.len() == k as usize {
            result.push(path.clone());
            return;
        }
        if step == n + 1 {
            return;
        }
        Self::backtrack(n, k, step + 1, path, result);
        path.push(step);
        Self::backtrack(n, k, step + 1, path, result);
        path.pop();
    }
}
```

### Go
```go
package main

var result [][]int

func combine(n int, k int) [][]int {
	result = make([][]int, 0)
	backtrack(n, k, 1, []int{})
	return result
}
func backtrack(n, k, step int, path []int) {
	if len(path) == k {
		snapshot := make([]int, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	if step == n+1 {
		return
	}
	backtrack(n, k, step+1, path)
	path = append(path, step)
	backtrack(n, k, step+1, path)
	path = path[:len(path)-1]
}

```

### JavaScript
```javascript
/**
 * @param {number} n
 * @param {number} k
 * @return {number[][]}
 */
var combine = function(n, k) {
    let result = []
    let path = []
    let backtrack = step => {
        if (path.length == k) {
            result.push([...path])
            return
        }
        if (step == n+1) {
            return
        }
        backtrack(step+1)
        path.push(step)
        backtrack(step+1)
        path.pop()
    }
    backtrack(1)
    return result
};
```

### Python
```python
class Solution:
    def __init__(self):
        self.result = []

    def combine(self, n: int, k: int) -> List[List[int]]:
        self.backtrack(n, k, 1, [])
        return self.result

    def backtrack(self, n, k, step, path):
        if len(path) == k:
            self.result.append(path)
            return
        if step == n + 1:
            return
        self.backtrack(n, k, step+1, path)
        path.append(step)
        self.backtrack(n,k,step+1, path)
        path.pop()

```

### C++
```c++
class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> combine(int n, int k) {
        vector<int> path;
        backtrack(n, k, 1, path);
        return result;
    }
    void backtrack(int n, int k, int step, vector<int>& path) {
        if (path.size() == k) {
            result.push_back(path);
            return;
        }
        if (step == n + 1) {
            return;
        }
        backtrack(n, k, step + 1, path);
        path.push_back(step);
        backtrack(n, k, step + 1, path);
        path.pop_back();
    }
};
```