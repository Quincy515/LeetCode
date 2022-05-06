# [1347.制造字母异位词的最小步骤数](https://leetcode-cn.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/description/)

给你两个长度相等的字符串 `s` 和 `t`。每一个步骤中，你可以选择将 `t` 中的 **任一字符** 替换为 **另一个字符**。

返回使 `t` 成为 `s` 的字母异位词的最小步骤数。

**字母异位词** 指字母相同，但排列不同（也可能相同）的字符串。

 

**示例 1：**

```
输出：s = "bab", t = "aba"
输出：1
提示：用 'b' 替换 t 中的第一个 'a'，t = "bba" 是 s 的一个字母异位词。
```

**示例 2：**

```
输出：s = "leetcode", t = "practice"
输出：5
提示：用合适的字符替换 t 中的 'p', 'r', 'a', 'i' 和 'c'，使 t 变成 s 的字母异位词。
```

**示例 3：**

```
输出：s = "anagram", t = "mangaar"
输出：0
提示："anagram" 和 "mangaar" 本身就是一组字母异位词。 
```

**示例 4：**

```
输出：s = "xxyyzz", t = "xxyyzz"
输出：0
```

**示例 5：**

```
输出：s = "friend", t = "family"
输出：4
```

 

**提示：**

- `1 <= s.length <= 50000`
- `s.length == t.length`
- `s` 和 `t` 只包含小写英文字母

------

[Discussion](https://leetcode-cn.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/comments/) | [Solution](https://leetcode-cn.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/solution/)

**思路** 

1、用两个哈希表分别映射两个字符串

2、枚举每个字母，对两个哈希表对应的字母进行差值，当差值为负数，则累加，最后返回这个答案

**题解**

```rust
use std::collections::HashMap;
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let (mut hashs, mut hasht) = (HashMap::new(), HashMap::new());
        let (s, t) = (
            s.chars().collect::<Vec<char>>(),
            t.chars().collect::<Vec<char>>(),
        );
        let mut result = 0;
        for i in 0..s.len() {
            *hashs.entry(s[i]).or_insert(0) += 1;
            *hasht.entry(t[i]).or_insert(0) += 1;
        }
        for i in 'a'..='z' {
            let x = *hashs.entry(i).or_insert(0) - *hasht.entry(i).or_insert(0);
            if x > 0 {
                result += x;
            }
        }

        result
    }
}
```



