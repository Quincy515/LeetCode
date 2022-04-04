# 面试题 17.11. 单词距离
有个内含单词的超大文本文件，给定任意两个单词，找出在这个文件中这两个单词的最短距离(相隔单词数)。如果寻找过程在这个文件中会重复多次，而每次寻找的单词不同，你能对此优化吗?

## 示例：
```
输入：words = ["I","am","a","student","from","a","university","in","a","city"], word1 = "a", word2 = "student"
输出：1
```

## 提示：

- words.length <= 100000


## 题解：
### Rust
```rust
impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        let (mut w1ps, mut w2ps) = (vec![], vec![]);
        for (i, word) in words.iter().enumerate() {
            if *word == word1 {
                w1ps.push(i as i32);
            } else if *word == word2 {
                w2ps.push(i as i32);
            }
        }
        let (mut p1, mut p2) = (0, 0);
        let mut min_ret = i32::MAX;
        while p1 < w1ps.len() && p2 < w2ps.len() {
            let (pos1, pos2) = (w1ps[p1], w2ps[p2]);
            if pos1 > pos2 {
                if min_ret > pos1 - pos2 {
                    min_ret = pos1 - pos2;
                }
                p2 += 1;
            } else {
                if min_ret > pos2 - pos1 {
                    min_ret = pos2 - pos1;
                }
                p1 += 1;
            }
        }

        min_ret
    }
}
```

### Go
```go
package main

import (
	"math"
)

func findClosest(words []string, word1 string, word2 string) int {
	w1ps := make([]int, 0)
	w2ps := make([]int, 0)
	for i := 0; i < len(words); i++ {
		word := words[i]
		if word == word1 {
			w1ps = append(w1ps, i)
		} else if word == word2 {
			w2ps = append(w2ps, i)
		}
	}
	p1 := 0
	p2 := 0
	minRet := math.MaxInt32
	for p1 < len(w1ps) && p2 < len(w2ps) {
		pos1 := w1ps[p1]
		pos2 := w2ps[p2]
		if pos1 > pos2 {
			if minRet > pos1-pos2 {
				minRet = pos1 - pos2
			}
			p2++
		} else {
			if minRet > pos2-pos1 {
				minRet = pos2 - pos1
			}
			p1++
		}
	}
	return minRet
}

```

### JavaScript
```javascript
/**
 * @param {string[]} words
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var findClosest = function(words, word1, word2) {
    let w1ps = []
    let w2ps = []
    for (let i = 0; i < words.length; ++i) {
        let word = words[i]
        if (word === word1) {
            w1ps.push(i)
        } else if (word === word2) {
            w2ps.push(i)
        }
    }
    let p1 = 0
    let p2 = 0
    let minRet = Number.MAX_SAFE_INTEGER
    while (p1 < w1ps.length && p2 < w2ps.length) {
        let pos1 = w1ps[p1]
        let pos2 = w2ps[p2]
        if (pos1 > pos2) {
            if (minRet > pos1-pos2) {
                minRet = pos1-pos2
            }
            p2++
        } else {
            if (minRet > pos2-pos1) {
                minRet = pos2-pos1
            }
            p1++
        }
    }
    return minRet
};
```

### Python
```python
class Solution:
    def findClosest(self, words: List[str], word1: str, word2: str) -> int:
        w1ps = []
        w2ps = []
        for i in range(len(words)):
            if words[i] == word1:
                w1ps.append(i)
            elif words[i] == word2:
                w2ps.append(i)
        p1 = 0
        p2 = 0
        minRet = float("inf")
        while p1 < len(w1ps) and p2 < len(w2ps):
            pos1 = w1ps[p1]
            pos2 = w2ps[p2]
            if pos1 > pos2:
                if minRet > pos1 - pos2:
                    minRet = pos1 - pos2
                p2 += 1
            else:
                if minRet > pos2 - pos1:
                    minRet = pos2 - pos1
                p1 += 1
        return minRet
```

### C++
```c++
class Solution {
public:
    int findClosest(vector<string>& words, string word1, string word2) {
        vector<int> w1ps;
        vector<int> w2ps;
        for (int i = 0; i < words.size(); ++i) {
            if (words[i] == word1) {
                w1ps.push_back(i);
            } else if (words[i] == word2) {
                w2ps.push_back(i);
            }
        }
        int p1 = 0;
        int p2 = 0;
        int minRet = INT_MAX;
        while (p1 < w1ps.size() && p2 < w2ps.size()) {
            if (w1ps[p1] < w2ps[p2]) {
                if (w2ps[p2] - w1ps[p1] < minRet) minRet = std::min(minRet, w2ps[p2] -
                                                                            w1ps[p1]);
                p1++;
            } else {
                if (w1ps[p1] - w2ps[p2] < minRet) minRet = std::min(minRet, w1ps[p1] -
                                                                            w2ps[p2]);
                p2++;
            }
        }
        return minRet;
    }
};
```