<p>给你一个整数数组 <code>nums</code> ，其中可能包含重复元素，请你返回该数组所有可能的子集（幂集）。</p>

<p>解集 <strong>不能</strong> 包含重复的子集。返回的解集中，子集可以按 <strong>任意顺序</strong> 排列。</p>

<div class="original__bRMd">
<div>
<p> </p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,2,2]
<strong>输出：</strong>[[],[1],[1,2],[1,2,2],[2],[2,2]]
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
</ul>
</div>
</div>
<div><div>Related Topics</div><div><li>位运算</li><li>数组</li><li>回溯</li></div></div><br>

## 题解
### Rust
```rust
use std::collections::HashMap;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut hm = HashMap::new();
        for num in &nums {
            let mut count = 1;
            if hm.contains_key(num) {
                count += hm.get(num).unwrap();
            }
            hm.insert(num, count);
        }
        let n = hm.len();
        let mut unique_nums = vec![0; n];
        let mut counts = vec![0; n];
        let mut k = 0;
        for num in &nums {
            if hm.contains_key(num) {
                unique_nums[k] = *num;
                counts[k] = *hm.get(num).unwrap();
                k += 1;
                hm.remove(num);
            }
        }
        Self::backtrack(&mut unique_nums, &mut counts, 0, &mut vec![], &mut result);
        result
    }

    fn backtrack(
        unique_nums: &mut Vec<i32>,
        counts: &mut Vec<i32>,
        k: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if k == unique_nums.len() as i32 {
            result.push(path.clone());
            return;
        }
        let mut count = 0;
        while count <= counts[k as usize] {
            for _ in 0..count {
                path.push(unique_nums[k as usize]);
            }
            Self::backtrack(unique_nums, counts, k + 1, path, result);
            for _ in 0..count {
                path.pop();
            }
            count += 1;
        }
    }
}

```

### Go
```go
package main

var result [][]int

func subsetsWithDup(nums []int) [][]int {
	result = make([][]int, 0)
	hm := make(map[int]int, 0)
	for i := 0; i < len(nums); i++ {
		count := 1
		if _, ok := hm[nums[i]]; ok {
			count += hm[nums[i]]
		}
		hm[nums[i]] = count
	}
	n := len(hm)
	uniqueNums := make([]int, n)
	counts := make([]int, n)
	k := 0
	for i := 0; i < len(nums); i++ {
		if _, ok := hm[nums[i]]; ok {
			uniqueNums[k] = nums[i]
			counts[k] = hm[nums[i]]
			k++
			delete(hm, nums[i])
		}
	}
	backtrack(uniqueNums, counts, 0, []int{})
	return result
}
func backtrack(uniqueNums, counts []int, k int, path []int) {
	if k == len(uniqueNums) {
		snapshot := make([]int, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	for count := 0; count <= counts[k]; count++ {
		for i := 0; i < count; i++ {
			path = append(path, uniqueNums[k])
		}
		backtrack(uniqueNums, counts, k+1, path)
		for i := 0; i < count; i++ {
			path = path[:len(path)-1]
		}
	}
}

```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var subsetsWithDup = function(nums) {
    let result = []
    let hm = new Map()
    for (let i = 0; i < nums.length; ++i) {
        let count = 1
        if (hm.has(nums[i])) {
            count += hm.get(nums[i])
        }
        hm.set(nums[i], count)
    }
    let n = hm.size
    let uniqueNums = new Array(n)
    let counts = new Array(n)
    let k = 0
    for (let i = 0; i < nums.length; ++i) {
        if (hm.has(nums[i])) {
            uniqueNums[k] = nums[i]
            counts[k] = hm.get(nums[i])
            k++
            hm.delete(nums[i])
        }
    }
    let path = []
    let backtrack = k => {
        if (k == uniqueNums.length) {
            result.push([...path])
            return
        }
        for (let count = 0; count <= counts[k]; ++count) {
            for (let i = 0; i < count; ++i) {
                path.push(uniqueNums[k])
            }
            backtrack(k+1)
            for (let i = 0; i < count; ++i) {
                path.pop()
            }
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

    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        hm = dict()
        for i in range(len(nums)):
            count = 1
            if nums[i] in hm:
                count += hm[nums[i]]
            hm[nums[i]] = count
        n = len(hm)
        uniqueNum = [None] * n
        counts = [None] * n
        k = 0
        for i in range(len(nums)):
            if nums[i] in hm:
                uniqueNum[k] = nums[i]
                counts[k] = hm[nums[i]]
                k +=1
                hm.pop(nums[i])
        self.backtrack(uniqueNum, counts, 0, [])
        return self.result

    def backtrack(self, uniqueNum, counts, k, path):
        if k == len(uniqueNum):
            self.result.append(path[:])
            return
        for count in range(counts[k]+1):
            for i in range(count):
                path.append(uniqueNum[k])
            self.backtrack(uniqueNum, counts, k+1, path)
            for i in range(count):
                path.pop()

```

### C++
```c++
class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> subsetsWithDup(vector<int>& nums) {
        unordered_map<int, int> hm;
        for (int i = 0; i < nums.size(); ++i) {
            int count = 1;
            auto it = hm.find(nums[i]);
            if (it != hm.end()) {
                count += it->second;
            }
            hm[nums[i]] = count;
        }
        int n = hm.size();
        vector<int> uniqueNums(n);
        vector<int> counts(n);
        int k = 0;
        for (int i = 0; i < nums.size(); ++i) {
            auto it = hm.find(nums[i]);
            if (it != hm.end()) {
                uniqueNums[k] = nums[i];
                counts[k] = it->second;
                k++;
                hm.erase(it);
            }
        }
        vector<int> path;
        backtrack(uniqueNums, counts, 0, path);
        return result;
    }
    void backtrack(vector<int> uniqueNums, vector<int> counts, int k, vector<int>&
    path) {
        if (k == uniqueNums.size()) {
            result.push_back(path);
            return;
        }
        for (int count = 0; count <= counts[k]; ++count) {
            for (int i = 0; i < count; ++i) {
                path.push_back(uniqueNums[k]);
            }
            backtrack(uniqueNums, counts, k + 1, path);
            for (int i = 0; i < count; ++i) {
                path.pop_back();
            }
        }
    }
};
```