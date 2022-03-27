<p>找出所有相加之和为&nbsp;<code>n</code><em> </em>的&nbsp;<code>k</code><strong>&nbsp;</strong>个数的组合，且满足下列条件：</p>

<ul>
	<li>只使用数字1到9</li>
	<li>每个数字&nbsp;<strong>最多使用一次</strong>&nbsp;</li>
</ul>

<p>返回 <em>所有可能的有效组合的列表</em> 。该列表不能包含相同的组合两次，组合可以以任何顺序返回。</p>

<p>&nbsp;</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> <em><strong>k</strong></em> = 3, <em><strong>n</strong></em> = 7
<strong>输出:</strong> [[1,2,4]]
<strong>解释:</strong>
1 + 2 + 4 = 7
没有其他符合的组合了。</pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> <em><strong>k</strong></em> = 3, <em><strong>n</strong></em> = 9
<strong>输出:</strong> [[1,2,6], [1,3,5], [2,3,4]]
<strong>解释:
</strong>1 + 2 + 6 = 9
1 + 3 + 5 = 9
2 + 3 + 4 = 9
没有其他符合的组合了。</pre>

<p><strong>示例 3:</strong></p>

<pre>
<strong>输入:</strong> k = 4, n = 1
<strong>输出:</strong> []
<strong>解释:</strong> 不存在有效的组合。
在[1,9]范围内使用4个不同的数字，我们可以得到的最小和是1+2+3+4 = 10，因为10 &gt; 1，没有有效的组合。
</pre>

<p>&nbsp;</p>

<p><strong>提示:</strong></p>

<ul>
	<li><code>2 &lt;= k &lt;= 9</code></li>
	<li><code>1 &lt;= n &lt;= 60</code></li>
</ul>
<div><div>Related Topics</div><div><li>数组</li><li>回溯</li></div></div><br>


## 题解

### Rust
```rust

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        // 1 ~ 9，选 k 个数，和为 n
        let mut result = vec![];
        Solution::backtrack(k, n, 1, 0, &mut vec![], &mut result);

        result
    }

    fn backtrack(
        k: i32,
        n: i32,
        step: i32,
        sum: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if sum == n && path.len() as i32 == k {
            result.push(path.clone());
            return;
        }
        if sum > n || path.len() as i32 > k || step > 9 {
            return;
        }
        Self::backtrack(k, n, step + 1, sum, path, result);
        path.push(step);
        Self::backtrack(k, n, step + 1, sum + step, path, result);
        path.pop();
    }
}
```

### Go

```go
package main

var result [][]int

func combinationSum3(k int, n int) [][]int {
	result = make([][]int, 0)
	backtrack(k, n, 1, 0, []int{})
	return result
}
func backtrack(k, n, step, sum int, path []int) {
	if sum == n && len(path) == k {
		snapshot := make([]int, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	if sum > n || len(path) > k || step > 9 {
		return
	}
	backtrack(k, n, step+1, sum, path)
	path = append(path, step)
	backtrack(k, n, step+1, sum+step, path)
	path = path[:len(path)-1]
}

```

### JavaScript

```javascript
/**
 * @param {number} k
 * @param {number} n
 * @return {number[][]}
 */
var combinationSum3 = function(k, n) {
    let result = []
    let path = []
    let backtrack = (step, sum) => {
        if (sum == n && path.length == k) {
            result.push([...path])
            return
        }
        if (sum > n || path.length > k || step > 9) {
            return
        }
        backtrack(step + 1, sum)
        path.push(step)
        backtrack(step + 1, sum + step)
        path.pop()
    }
    backtrack(1, 0)
    return result
};
```

### Python
```python
class Solution:
    def __init__(self):
        self.result = []

    def combinationSum3(self, k: int, n: int) -> List[List[int]]:
        self.backtrack(k,n,1,0,[])
        return self.result

    def backtrack(self, k, n, step, total, path):
        if total == n and len(path) == k:
            self.result.append(path[:])
            return
        if total > n or len(path) > k or step > 9:
            return
        self.backtrack(k,n,step+1,total,path)
        path.append(step)
        self.backtrack(k,n,step+1,total+step,path)
        path.pop()

```
### C++
```c++
class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> combinationSum3(int k, int n) {
        vector<int> path;
        backtrack(k, n, 1, 0, path);
        return result;
    }
    void backtrack(int k, int n, int step, int sum, vector<int> &path) {
        if (sum == n && path.size() == k) {
            result.push_back(path);
            return;
        }
        if (sum > n || path.size() > k || step > 9) {
            return;
        }
        backtrack(k, n, step + 1, sum, path);
        path.push_back(step);
        backtrack(k, n, step + 1, sum + step, path);
        path.pop_back();
    }
};
```