<p>给你一个 <strong>无重复元素</strong> 的整数数组&nbsp;<code>candidates</code> 和一个目标整数&nbsp;<code>target</code>&nbsp;，找出&nbsp;<code>candidates</code>&nbsp;中可以使数字和为目标数&nbsp;<code>target</code> 的 <em>所有&nbsp;</em><strong>不同组合</strong> ，并以列表形式返回。你可以按 <strong>任意顺序</strong> 返回这些组合。</p>

<p><code>candidates</code> 中的 <strong>同一个</strong> 数字可以 <strong>无限制重复被选取</strong> 。如果至少一个数字的被选数量不同，则两种组合是不同的。&nbsp;</p>

<p>对于给定的输入，保证和为&nbsp;<code>target</code> 的不同组合数少于 <code>150</code> 个。</p>

<p>&nbsp;</p>

<p><strong>示例&nbsp;1：</strong></p>

<pre>
<strong>输入：</strong>candidates = <code>[2,3,6,7], </code>target = <code>7</code>
<strong>输出：</strong>[[2,2,3],[7]]
<strong>解释：</strong>
2 和 3 可以形成一组候选，2 + 2 + 3 = 7 。注意 2 可以使用多次。
7 也是一个候选， 7 = 7 。
仅有这两种组合。</pre>

<p><strong>示例&nbsp;2：</strong></p>

<pre>
<strong>输入: </strong>candidates = [2,3,5]<code>, </code>target = 8
<strong>输出: </strong>[[2,2,2,2],[2,3,3],[3,5]]</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入: </strong>candidates = <code>[2], </code>target = 1
<strong>输出: </strong>[]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= candidates.length &lt;= 30</code></li>
	<li><code>1 &lt;= candidates[i] &lt;= 200</code></li>
	<li><code>candidate</code> 中的每个元素都 <strong>互不相同</strong></li>
	<li><code>1 &lt;= target &lt;= 500</code></li>
</ul>
<div><div>Related Topics</div><div><li>数组</li><li>回溯</li></div></div><br>

## 题解
### Rust
```rust
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        Self::backtrack(&candidates, 0, target, &mut vec![], &mut result);

        result
    }

    fn backtrack(
        candidates: &Vec<i32>,
        k: i32,
        left: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if left == 0 {
            result.push(path.clone());
            return;
        }
        if k == candidates.len() as i32 {
            return;
        }
        for i in 0..=left / candidates[k as usize] {
            for _ in 0..i {
                path.push(candidates[k as usize]);
            }
            Self::backtrack(
                candidates,
                k + 1,
                left - candidates[k as usize] * i,
                path,
                result,
            );
            for _ in 0..i {
                path.pop();
            }
        }
    }
}
```

### Go
```go
package main

var result [][]int

func combinationSum(candidates []int, target int) [][]int {
	result = make([][]int, 0)
	backtrack(candidates, 0, target, []int{})
	return result
}
func backtrack(candidates []int, k, left int, path []int) {
	if left == 0 {
		snapshot := make([]int, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	if k == len(candidates) {
		return
	}
	for i := 0; i <= left/candidates[k]; i++ {
		for j := 0; j < i; j++ {
			path = append(path, candidates[k])
		}
		backtrack(candidates, k+1, left-i*candidates[k], path)
		for j := 0; j < i; j++ {
			path = path[:len(path)-1]
		}
	}
}

```

### JavaScript
```javascript
/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum = function(candidates, target) {
    let result = []
    let path = []
    let backtrack = (k, left) => {
        if (left == 0) {
            result.push([...path])
            return
        }
        if (k == candidates.length) {
            return
        }
        for (let i = 0; i <= Math.floor(left/candidates[k]); ++i) {
            for (let j = 0; j < i; ++j) {
                path.push(candidates[k])
            }
            backtrack(k+1, left-i*candidates[k])
            for (let j = 0; j < i; ++j) {
                path.pop()
            }
        }
    }
    backtrack(0, target)
    return result
};
```

### Python
```python
class Solution:
    def __init__(self):
        self.result = []

    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        self.backtrack(candidates,0,target,[])
        return self.result

    def backtrack(self, candidates, k, left, path):
        if left == 0:
            self.result.append(path[:])
            return
        if k == len(candidates):
            return
        for i in range(left //candidates[k] +1):
            for j in range(i):
                path.append(candidates[k])
            self.backtrack(candidates, k+1, left - candidates[k] * i, path)
            for j in range(i):
                path.pop()

```

### C++
```c++
class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<int> path;
        backtrack(candidates, 0, target, path);
        return result;
    }
    void backtrack(vector<int> candidates, int k, int left, vector<int> &path) {
        if (left == 0) {
            result.push_back(path);
            return;
        }
        if (k == candidates.size()) {
            return;
        }
        for (int i = 0; i <= left / candidates[k]; ++i) {
            for (int j = 0; j < i; ++j) {
                path.push_back(candidates[k]);
            }
            backtrack(candidates, k + 1, left - i * candidates[k], path);
            for (int j = 0; j < i; ++j) {
                path.pop_back();
            }
        }
    }
};
```