<p>给定一个不含重复数字的数组 <code>nums</code> ，返回其 <em>所有可能的全排列</em> 。你可以 <strong>按任意顺序</strong> 返回答案。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,2,3]
<strong>输出：</strong>[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [0,1]
<strong>输出：</strong>[[0,1],[1,0]]
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>nums = [1]
<strong>输出：</strong>[[1]]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 6</code></li>
	<li><code>-10 &lt;= nums[i] &lt;= 10</code></li>
	<li><code>nums</code> 中的所有整数 <strong>互不相同</strong></li>
</ul>
<div><div>Related Topics</div><div><li>数组</li><li>回溯</li></div></div><br>

## 题解
### Rust
```rust
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        Self::backtrack(&nums, 0, &mut vec![], &mut result);
        result
    }

    // 路径：记录在 path 中
    // 决策阶段：k
    // 可选列表：nums 中除掉存在与 path 中的数据
    fn backtrack(nums: &[i32], k: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        // 结束条件
        if k == nums.len() as i32 {
            result.push(path.clone());
            return;
        }
        for num in nums {
            if path.contains(num) {
                continue;
            }
            // 做选择
            path.push(*num);
            // 递归
            Self::backtrack(nums, k + 1, path, result);
            // 撤销选择
            path.pop();
        }
    }
}
```

### Go
```go
package main

var result [][]int

func permute(nums []int) [][]int {
	result = make([][]int, 0)
	path := make([]int, 0)
	backtrack(nums, 0, path)
	return result
}
func backtrack(nums []int, k int, path []int) {
	if k == len(nums) {
		snapshot := make([]int, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	for i := 0; i < len(nums); i++ {
		if contain(path, nums[i]) {
			continue
		}
		path = append(path, nums[i])
		backtrack(nums, k+1, path)
		path = path[:len(path)-1]
	}
}
func contain(path []int, num int) bool {
	for _, p := range path {
		if p == num {
			return true
		}
	}
	return false
}

```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permute = function(nums) {
    let result = []
    let path = []
    let backtrack = k => {
        if (k == nums.length) {
            result.push([...path])
            return
        }
        for (let i = 0; i < nums.length; ++i) {
            if (path.includes(nums[i])) {
                continue
            }
            path.push(nums[i])
            backtrack(k+1)
            path.pop()
        }
    }
    backtrack(0)
    return result
};
```

### Python
```python
class Solution:
    def __init__(self):
        self.result = []

    def permute(self, nums: List[int]) -> List[List[int]]:
        self.backtrack(nums, 0, [])
        return self.result

    def backtrack(self, nums, k, path):
        if k == len(nums):
            self.result.append(path[:])
            return
        for i in range(len(nums)):
            if nums[i] in path:
                continue
            path.append(nums[i])
            self.backtrack(nums, k+1,path)
            path.pop()

```

### Cpp
```c++
class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> permute(vector<int>& nums) {
        vector<int> path;
        backtrack(nums, 0, path);
        return result;
    }
    void backtrack(vector<int> nums, int k, vector<int> &path) {
        if (k == nums.size()) {
            result.push_back(path);
        }
        for (int i = 0; i < nums.size(); ++i) {
            if (find(path.begin(), path.end(), nums[i]) != path.end()) {
                continue;
            }
            path.push_back(nums[i]);
            backtrack(nums, k + 1, path);
            path.pop_back();
        }
    }
};
```