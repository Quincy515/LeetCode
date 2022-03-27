<p>给定一个可包含重复数字的序列 <code>nums</code> ，<em><strong>按任意顺序</strong></em> 返回所有不重复的全排列。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,1,2]
<strong>输出：</strong>
[[1,1,2],
 [1,2,1],
 [2,1,1]]
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,2,3]
<strong>输出：</strong>[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 8</code></li>
	<li><code>-10 &lt;= nums[i] &lt;= 10</code></li>
</ul>
<div><div>Related Topics</div><div><li>数组</li><li>回溯</li></div></div><br>

## 题解
### Rust
```rust
use std::collections::HashMap;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
        Solution::backtrack(
            &mut unique_nums,
            &mut counts,
            0,
            &mut vec![],
            nums.len(),
            &mut result,
        );
        result
    }

    fn backtrack(
        unique_nums: &mut Vec<i32>,
        counts: &mut Vec<i32>,
        k: i32,
        path: &mut Vec<i32>,
        n: usize,
        result: &mut Vec<Vec<i32>>,
    ) {
        if k == n as i32 {
            result.push(path.clone());
            return;
        }
        for i in 0..unique_nums.len() {
            if counts[i] == 0 {
                continue;
            }
            path.push(unique_nums[i]); // 添加选择
            counts[i] -= 1;
            Self::backtrack(unique_nums, counts, k + 1, path, n, result);
            path.pop(); // 撤销选择
            counts[i] += 1;
        }
    }
}
```

### Go
```go
package main

var result [][]int

func permuteUnique(nums []int) [][]int {
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
	backtrack(uniqueNums, counts, 0, []int{}, len(nums))
	return result
}
func backtrack(uniqueNums, counts []int, k int, path []int, n int) {
	if k == n {
		snapshot := make([]int, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	for i := 0; i < len(uniqueNums); i++ {
		if counts[i] == 0 {
			continue
		}
		path = append(path, uniqueNums[i])
		counts[i]--
		backtrack(uniqueNums, counts, k+1, path, n)
		path = path[:len(path)-1]
		counts[i]++
	}
}

```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permuteUnique = function(nums) {
    let result = []
    let hm = new Map()
    for (let num of nums) {
        let count = 1
        if (hm.has(num)) {
            count += hm.get(num)
        }
        hm.set(num, count)
    }
    let n = hm.size
    let uniqueNums = new Array(n)
    let counts = new Array(n)
    let k = 0
    for (let num of nums) {
        if (hm.has(num)){
            uniqueNums[k] = num
            counts[k] = hm.get(num)
            k++
            hm.delete(num)
        }
    }
    let path = []
    let backtrack = k => {
        if (k == nums.length) {
            result.push([...path])
            return
        }
        for (let i = 0; i < uniqueNums.length; ++i) {
            if (counts[i] == 0) {
                continue
            }
            path.push(uniqueNums[i])
            counts[i] -= 1
            backtrack(k+1)
            path.pop()
            counts[i] += 1
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

    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
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
        self.backtrack(uniqueNum, counts, 0, [], len(nums))
        return self.result

    def backtrack(self, uniqueNum, counts, k, path,n):
        if k == n:
            self.result.append(path[:])
            return
        for i in range(len(uniqueNum)):
            if counts[i] == 0:
                continue
            path.append(uniqueNum[i])
            counts[i] -= 1
            self.backtrack(uniqueNum, counts, k+1, path, n)
            path.pop()
            counts[i] += 1

```

### Cpp
```c++
class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> permuteUnique(vector<int>& nums) {
        unordered_map<int, int> hm;
        for (int i = 0; i < nums.size(); ++i) {
            int count = 1;
            auto it = hm.find(nums[i]);
            if (it != hm.end()) {
                count += hm[nums[i]];
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
                counts[k] = hm[nums[i]];
                k++;
                hm.erase(it);
            }
        }
        vector<int> path;
        backtrack(uniqueNums, counts, 0, path, nums.size());
        return result;
    }
    void backtrack(vector<int> uniqueNums, vector<int> counts, int k, vector<int> &path, int n) {
        if (k == n) {
            result.push_back(path);
            return;
        }
        for (int i = 0; i < uniqueNums.size(); ++i) {
            if (counts[i] == 0) continue;
            path.push_back(uniqueNums[i]);
            counts[i]--;
            backtrack(uniqueNums, counts, k + 1, path, n);
            path.pop_back();
            counts[i]++;
        }
    }
};
```