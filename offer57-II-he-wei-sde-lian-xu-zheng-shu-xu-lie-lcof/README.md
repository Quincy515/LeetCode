# 剑指 Offer 57 - II. 和为s的连续正数序列
输入一个正整数 target ，输出所有和为 target 的连续正整数序列（至少含有两个数）。

序列内的数字由小到大排列，不同序列按照首个数字从小到大排列。



## 示例 1：

输入：target = 9
输出：[[2,3,4],[4,5]]

## 示例 2：

输入：target = 15
输出：[[1,2,3,4,5],[4,5,6],[7,8]]


## 限制：

- 1 <= target <= 10^5

## 题解：
### Rust
```rust
impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let (mut p, mut q, mut sum) = (1, 2, 3);
        while p < q {
            if sum == target {
                let mut arr = vec![0; q - p + 1];
                for i in p..=q {
                    arr[i - p] = i as i32;
                }
                result.push(arr);
                sum -= p as i32;
                p += 1;
                q += 1;
                sum += q as i32;
            } else if sum > target {
                sum -= p as i32;
                p += 1;
            } else {
                q += 1;
                sum += q as i32;
            }
        }

        let mut result_arr: Vec<Vec<i32>> = vec![vec![]; result.len()];
        for i in 0..result.len() {
            result_arr[i] = result[i].clone();
        }
        result_arr[..result.len()].clone_from_slice(&result[..]);
        result_arr
    }
}
```

### Go
```go
package main

func findContinuousSequence(target int) [][]int {
	result := make([][]int, 0)
	p := 1
	q := 2
	sum := 3
	for p < q {
		if sum == target {
			arr := make([]int, q-p+1)
			for i := p; i <= q; i++ {
				arr[i-p] = i
			}
			result = append(result, arr)
			sum -= p
			p++
			q++
			sum += q
		} else if sum > target {
			sum -= p
			p++
		} else {
			q++
			sum += q
		}
	}
	return result
}
```

### JavaScript
```javascript

/**
 * @param {number} target
 * @return {number[][]}
 */
var findContinuousSequence = function(target) {
    let result = []
    let p = 1
    let q = 2
    let sum = 3
    while (p < q) {
        if (sum === target) {
            let arr = new Array(q-p+1)
            for (let i = p; i <= q; ++i) {
                arr[i-p] = i
            }
            result.push(arr)
            sum -= p
            p++
            q++
            sum += q
        } else if (sum > target) {
            sum -= p
            p++
        } else {
            q++
            sum += q
        }
    }
    return result
};
```

### Python
```python
class Solution:
    def findContinuousSequence(self, target: int) -> List[List[int]]:
        result = []
        p = 1
        q = 2
        total = 3
        while p < q:
            arr = [0] * (q - p + 1)
            if total == target:
                for i in range(p, q+1):
                    arr[i-p] = i
                result.append(arr)
                total -= p
                p += 1
                q += 1
                total += q
            elif total > target:
                total -= p
                p += 1
            else:
                q += 1
                total += q
        return result
```

### C++
```c++
class Solution {
public:
    vector<vector<int>> findContinuousSequence(int target) {
        vector<vector<int>> res;
        int i = 1;
        int j = 2;
        int sum = 3;
        while (i != j) {
            if (sum == target) {
                vector<int> tmp;
                for (int k = i; k <= j; ++k) {
                    tmp.push_back(k);
                }
                res.push_back(tmp);
                sum -= i;
                i++;
                j++;
                sum += j;
            } else if (sum > target) {
                sum -= i;
                i++;
            } else {
                j++;
                sum += j;
            }
        }
        return res;
    }
};
```