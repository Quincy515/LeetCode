# [443.压缩字符串](https://leetcode-cn.com/problems/string-compression/description/)


给你一个字符数组 `chars` ，请使用下述算法压缩：

从一个空字符串 `s` 开始。对于 `chars` 中的每组 **连续重复字符** ：

- 如果这一组长度为 `1` ，则将字符追加到 `s` 中。
- 否则，需要向 `s` 追加字符，后跟这一组的长度。

压缩后得到的字符串 `s` **不应该直接返回** ，需要转储到字符数组 `chars` 中。需要注意的是，如果组长度为 `10` 或 `10` 以上，则在 `chars` 数组中会被拆分为多个字符。

请在 **修改完输入数组后** ，返回该数组的新长度。

你必须设计并实现一个只使用常量额外空间的算法来解决此问题。

 

**示例 1：**

```
输入：chars = ["a","a","b","b","c","c","c"]
输出：返回 6 ，输入数组的前 6 个字符应该是：["a","2","b","2","c","3"]
解释："aa" 被 "a2" 替代。"bb" 被 "b2" 替代。"ccc" 被 "c3" 替代。
```

**示例 2：**

```
输入：chars = ["a"]
输出：返回 1 ，输入数组的前 1 个字符应该是：["a"]
解释：唯一的组是“a”，它保持未压缩，因为它是一个字符。
```

**示例 3：**

```
输入：chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
输出：返回 4 ，输入数组的前 4 个字符应该是：["a","b","1","2"]。
解释：由于字符 "a" 不重复，所以不会被压缩。"bbbbbbbbbbbb" 被 “b12” 替代。
```

 

**提示：**

- `1 <= chars.length <= 2000`
- `chars[i]` 可以是小写英文字母、大写英文字母、数字或符号

------

[Discussion](https://leetcode-cn.com/problems/string-compression/comments/) | [Solution](https://leetcode-cn.com/problems/string-compression/solution/)

**原理**

1、由于 `chars[i]` 可以是小写英文字母、大写英文字母、数字或符号，所以可以在字符串结尾加上一个 '';

2、记录上一个字符，每次判断和上一个字符是否相等，如果相等则计数器加一；如果不相等则根据计数器的个数分情况讨论：

- 如果计数器个数为1，则在原数组上直接加一个字符即可
- 如果计数器不为1，则将原计数器转换成字符串以后，再加入到原字符数组中

**题解**

```rust
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut s: Vec<char> = Vec::new();
        s.push(chars[0]);
        let mut count = 0;
        for c in chars.iter() {
            if *c == s[s.len() - 1] {
                count += 1;
            } else {
                if count > 1 {
                    s.append(&mut count.to_string().chars().collect::<Vec<char>>());
                }
                s.push(*c);
                count = 1;
            }
        }
        if count > 1 {
            s.append(&mut count.to_string().chars().collect::<Vec<char>>());
        }
        chars.clear();
        chars.append(&mut s);
        chars.len() as i32
    }
}
```

