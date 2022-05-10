<p>给定一个候选人编号的集合&nbsp;<code>candidates</code>&nbsp;和一个目标数&nbsp;<code>target</code>&nbsp;，找出&nbsp;<code>candidates</code>&nbsp;中所有可以使数字和为&nbsp;<code>target</code>&nbsp;的组合。</p>

<p><code>candidates</code>&nbsp;中的每个数字在每个组合中只能使用&nbsp;<strong>一次</strong>&nbsp;。</p>

<p><strong>注意：</strong>解集不能包含重复的组合。&nbsp;</p>

<p>&nbsp;</p>

<p><strong>示例&nbsp;1:</strong></p>

<pre>
<strong>输入:</strong> candidates =&nbsp;<code>[10,1,2,7,6,1,5]</code>, target =&nbsp;<code>8</code>,
<strong>输出:</strong>
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]</pre>

<p><strong>示例&nbsp;2:</strong></p>

<pre>
<strong>输入:</strong> candidates =&nbsp;[2,5,2,1,2], target =&nbsp;5,
<strong>输出:</strong>
[
[1,2,2],
[5]
]</pre>

<p>&nbsp;</p>

<p><strong>提示:</strong></p>

<ul>
	<li><code>1 &lt;=&nbsp;candidates.length &lt;= 100</code></li>
	<li><code>1 &lt;=&nbsp;candidates[i] &lt;= 50</code></li>
	<li><code>1 &lt;= target &lt;= 30</code></li>
</ul>
<div><div>Related Topics</div><div><li>数组</li><li>回溯</li></div></div><br>

## 题解
### Rust
```rust
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut hash_table = HashMap::new();
        for num in &candidates {
            if !hash_table.contains_key(num) {
                hash_table.insert(num, 1);
            } else {
                hash_table.insert(num, hash_table[num] + 1);
            }
        }
        let mut nums = vec![];
        let mut counts = vec![];
        for num in &candidates {
            if hash_table.contains_key(num) {
                nums.push(*num);
                counts.push(*hash_table.get(num).unwrap());
                hash_table.remove(num);
            }
        }
        Solution::backtrack(&mut nums, &mut counts, 0, target, &mut vec![], &mut result);
        result
    }

    fn backtrack(
        nums: &mut Vec<i32>,
        counts: &mut Vec<i32>,
        k: i32,
        left: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if left == 0 {
            result.push(path.clone());
            return;
        }
        if left < 0 || k == nums.len() as i32 {
            return;
        }
        let mut count = 0;
        while count <= counts[k as usize] {
            for _ in 0..count {
                path.push(nums[k as usize]);
            }
            Self::backtrack(
                nums,
                counts,
                k + 1,
                left - count * nums[k as usize],
                path,
                result,
            );
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

func combinationSum2(candidates []int, target int) [][]int {
	result = make([][]int, 0)
	hashTable := make(map[int]int, 0)
	for i := 0; i < len(candidates); i++ {
		if _, ok := hashTable[candidates[i]]; !ok {
			hashTable[candidates[i]] = 1
		} else {
			hashTable[candidates[i]] = hashTable[candidates[i]] + 1
		}
	}
	nums := make([]int, 0)
	counts := make([]int, 0)
	for i := 0; i < len(candidates); i++ {
		if _, ok := hashTable[candidates[i]]; ok {
			nums = append(nums, candidates[i])
			counts = append(counts, hashTable[candidates[i]])
			delete(hashTable, candidates[i])
		}
	}
	backtrack(nums, counts, 0, target, []int{})
	return result
}
func backtrack(nums, counts []int, k, left int, path []int) {
	if left == 0 {
		snapshot := make([]int, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	if left < 0 || k == len(nums) {
		return
	}
	for count := 0; count <= counts[k]; count++ {
		for i := 0; i < count; i++ {
			path = append(path, nums[k])
		}
		backtrack(nums, counts, k+1, left-count*nums[k], path)
		for i := 0; i < count; i++ {
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
var combinationSum2 = function(candidates, target) {
    let result = []
    let hashTable = new Map()
    for (const candidate of candidates) {
        if (!hashTable.has(candidate)) {
            hashTable.set(candidate, 1)
        } else {
            hashTable.set(candidate, hashTable.get(candidate) + 1)
        }
    }
    let nums = []
    let counts = []
    for (const candidate of candidates) {
        if (hashTable.has(candidate)) {
            nums.push(candidate)
            counts.push(hashTable.get(candidate))
            hashTable.delete(candidate)
        }
    }
    let path = []
    let backtrack = (k, left) => {
        if (left == 0) {
            result.push([...path])
            return
        }
        if (left < 0 || k == nums.length) {
            return
        }
        for (let count = 0; count <= counts[k]; ++count) {
            for (let i = 0; i < count; ++i) {
                path.push(nums[k])
            }
            backtrack(k+1, left-count*nums[k])
            for (let i = 0; i < count; ++i) {
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

    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        hashTable = dict()
        for i in range(len(candidates)):
            if candidates[i] not in hashTable:
                hashTable[candidates[i]] = 1
            else:
                hashTable[candidates[i]] += 1
        nums = []
        counts = []
        for i in range(len(candidates)):
            if candidates[i] in hashTable:
                nums.append(candidates[i])
                counts.append(hashTable[candidates[i]])
                hashTable.pop(candidates[i])

        self.backtrack(nums, counts, 0, target, [])
        return self.result

    def backtrack(self, nums, counts, k, left, path):
        if left == 0:
            self.result.append(path[:])
            return
        if k == len(nums) or left < 0:
            return
        for count in range(counts[k] +1):
            for i in range(count):
                path.append(nums[k])
            self.backtrack(nums, counts, k+1, left - count*nums[k], path)
            for i in range(count):
                path.pop()
```
### C++
```c++
class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        unordered_map<int, int> hashTable;
        for (int i = 0; i < candidates.size(); ++i) {
            auto it = hashTable.find(candidates[i]);
            if (it == hashTable.end()) {
                hashTable[candidates[i]] = 1;
            } else {
                hashTable[candidates[i]] = hashTable[candidates[i]] + 1;
            }
        }
        vector<int> nums;
        vector<int> counts;
        for (int i = 0; i < candidates.size(); ++i) {
            auto it = hashTable.find(candidates[i]);
            if (it != hashTable.end()) {
                nums.push_back(candidates[i]);
                counts.push_back(hashTable[candidates[i]]);
                hashTable.erase(it);
            }
        }
        vector<int> path;
        backtrack(nums, counts, 0, target, path);
        return result;
    }
    void backtrack(vector<int> nums, vector<int> counts, int k, int left, vector<int> &path) {
        if (left == 0) {
            result.push_back(path);
            return;
        }
        if (left < 0 || k == nums.size()) {
            return;
        }
        for (int count = 0; count <= counts[k]; ++count) {
            for (int i = 0; i < count; ++i) {
                path.push_back(nums[k]);
            }
            backtrack(nums, counts, k + 1, left - count * nums[k], path);
            for (int i = 0; i < count; ++i) {
                path.pop_back();
            }
        }
    }
};
```