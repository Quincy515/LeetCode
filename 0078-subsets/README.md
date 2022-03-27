<p>给你一个整数数组 <code>nums</code> ，数组中的元素 <strong>互不相同</strong> 。返回该数组所有可能的子集（幂集）。</p>

<p>解集 <strong>不能</strong> 包含重复的子集。你可以按 <strong>任意顺序</strong> 返回解集。</p>

<p> </p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,2,3]
<strong>输出：</strong>[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [0]
<strong>输出：</strong>[[],[0]]
</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= nums.length <= 10</code></li>
	<li><code>-10 <= nums[i] <= 10</code></li>
	<li><code>nums</code> 中的所有元素 <strong>互不相同</strong></li>
</ul>
<div><div>Related Topics</div><div><li>位运算</li><li>数组</li><li>回溯</li></div></div><br>

## 题解
### Rust
```rust
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        Solution::backtrack(&nums, 0, &mut vec![], &mut result);
        result
    }

    // k 阶段
    // path 路径
    // nums[k] 选或不选 - 可选列表
    fn backtrack(nums: &Vec<i32>, k: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if k == nums.len() as i32 {
            result.push(path.clone());
            return;
        }
        Self::backtrack(nums, k + 1, path, result);
        path.push(nums[k as usize]);
        Self::backtrack(nums, k + 1, path, result);
        path.pop();
    }
}
```

### Go
```go
```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var subsets = function(nums) {
    let result = []
    let path = []
    let backtrack = k => {
        if (k == nums.length) {
            result.push([...path])
            return
        }
        backtrack(k+1)
        path.push(nums[k])
        backtrack(k+1)
        path.pop()
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

    def subsets(self, nums: List[int]) -> List[List[int]]:
        self.backtrack(nums, 0, [])
        return self.result

    def backtrack(self, nums, k, path):
        if k == len(nums):
            self.result.append(path[:])
            return
        self.backtrack(nums, k+1,path)
        path.append(nums[k])
        self.backtrack(nums, k+1,path)
        path.pop()

```

### C++
```c++
class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> subsets(vector<int>& nums) {
        vector<int> path;
        backtrack(nums, 0, path);
        return result;
    }
    void backtrack(vector<int> nums, int k, vector<int>& path) {
        if (k == nums.size()) {
            // 批注：C++不需要snapshot，push_back()内部会拷⻉⼀份数据的副本
            result.push_back(path);
            return;
        }
        backtrack(nums, k + 1, path);
        path.push_back(nums[k]);
        backtrack(nums, k + 1, path);
        path.pop_back();
    }
};
```