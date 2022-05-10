# [构造 K 个回文字符串](https://leetcode-cn.com/problems/construct-k-palindrome-strings/description/)

给你一个字符串 `s` 和一个整数 `k` 。请你用 `s` 字符串中 **所有字符** 构造 `k` 个非空 **回文串** 。

如果你可以用 `s` 中所有字符构造 `k` 个回文字符串，那么请你返回 **True** ，否则返回 **False** 。

 

**示例 1：**

```
输入：s = "annabelle", k = 2
输出：true
解释：可以用 s 中所有字符构造 2 个回文字符串。
一些可行的构造方案包括："anna" + "elble"，"anbna" + "elle"，"anellena" + "b"
```

**示例 2：**

```
输入：s = "leetcode", k = 3
输出：false
解释：无法用 s 中所有字符构造 3 个回文串。
```

**示例 3：**

```
输入：s = "true", k = 4
输出：true
解释：唯一可行的方案是让 s 中每个字符单独构成一个字符串。
```

**示例 4：**

```
输入：s = "yzyzyzyzyzyzyzy", k = 2
输出：true
解释：你只需要将所有的 z 放在一个字符串中，所有的 y 放在另一个字符串中。那么两个字符串都是回文串。
```

**示例 5：**

```
输入：s = "cr", k = 7
输出：false
解释：我们没有足够的字符去构造 7 个回文串。
```

 

**提示：**

- `1 <= s.length <= 10^5`
- `s` 中所有字符都是小写英文字母。
- `1 <= k <= 10^5`

------

[Discussion](https://leetcode-cn.com/problems/construct-k-palindrome-strings/comments/) | [Solution](https://leetcode-cn.com/problems/construct-k-palindrome-strings/solution/)

**题解**

```rust
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize { return false; }
        if s.len() == k as usize { return true; }
        // 1. 统计每个字符出现的频次
        let mut m = HashMap::new();
        for i in s.chars() {
            let count = m.entry(i).or_insert(0);
            *count += 1;
        }
		// 2. 统计出现奇数次字符的个数
        let mut odd = 0;
        for  v in m.values() {
            if v % 2 == 1 {
                odd += 1;
            }
        }
		// 3. 判断有多个在一个回文串中是无用的，因为回文串中只能放一个落单的字符，就是把它放在正中心
        if k < odd { return false; }
        true

    }
}
```

```go
package main

func canConstruct(s string, k int) bool {
	if len(s) < k {
		return false
	}
	if len(s) == k {
		return true
	}
	// 1. 统计每个字符出现的频次
	m := make(map[rune]int)
	for _, c := range s {
		m[c]++
	}
	// 2. 统计出现奇数次字符的个数
	var oddNumCnt int
	for _, v := range m {
		if v%2 == 1 {
			oddNumCnt++
		}
	}
	// 3. 判断有多个在一个回文串中是无用的，因为回文串中只能放一个落单的字符，就是把它放在正中心
	if k <= (oddNumCnt - 1) {
		return false
	}
	return true
}
```

