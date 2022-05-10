# [分割平衡字符串](https://leetcode-cn.com/problems/split-a-string-in-balanced-strings/description/)

在一个 **平衡字符串** 中，`'L'` 和 `'R'` 字符的数量是相同的。

给你一个平衡字符串 `s`，请你将它分割成尽可能多的平衡字符串。

**注意：**分割得到的每个字符串都必须是平衡字符串，且分割得到的平衡字符串是原平衡字符串的连续子串。

返回可以通过分割得到的平衡字符串的 **最大数量** **。**

 

**示例 1：**

```
输入：s = "RLRRLLRLRL"
输出：4
解释：s 可以分割为 "RL"、"RRLL"、"RL"、"RL" ，每个子字符串中都包含相同数量的 'L' 和 'R' 。
```

**示例 2：**

```
输入：s = "RLLLLRRRLR"
输出：3
解释：s 可以分割为 "RL"、"LLLRRR"、"LR" ，每个子字符串中都包含相同数量的 'L' 和 'R' 。
```

**示例 3：**

```
输入：s = "LLLLRRRR"
输出：1
解释：s 只能保持原样 "LLLLRRRR".
```

**示例 4：**

```
输入：s = "RLRRRLLRLL"
输出：2
解释：s 可以分割为 "RL"、"RRRLLRLL" ，每个子字符串中都包含相同数量的 'L' 和 'R' 。
```

 

**提示：**

- `1 <= s.length <= 1000`
- `s[i] = 'L' 或 'R'`
- `s` 是一个 **平衡** 字符串

------

[Discussion](https://leetcode-cn.com/problems/split-a-string-in-balanced-strings/comments/) | [Solution](https://leetcode-cn.com/problems/split-a-string-in-balanced-strings/solution/)

**思路**

1、由于本身一定是一个平衡字符串，所以一定可以通过切割转换成 **n** 个平衡字符串。

2、利用一个统计变量，线性扫描整个字符串，遇到 **L** 则加一，遇到 **R** 则减一，当统计变量为零的时候，贪心的切一刀，计数器加一。

3、返回这个计数器

**题解**

```rust
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let (mut count, mut result) = (0, 0);
        for i in s.chars() {
            if i == 'L' {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                result += 1;
            }
        }
        result
    }
}
```

