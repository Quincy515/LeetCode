# [1839.所有元音按顺序排布的最长子字符串](https://leetcode-cn.com/problems/longest-substring-of-all-vowels-in-order/description/)


当一个字符串满足如下条件时，我们称它是 **美丽的** ：

- 所有 5 个英文元音字母（`'a'` ，`'e'` ，`'i'` ，`'o'` ，`'u'`）都必须 **至少** 出现一次。
- 这些元音字母的顺序都必须按照 **字典序** 升序排布（也就是说所有的 `'a'` 都在 `'e'` 前面，所有的 `'e'` 都在 `'i'` 前面，以此类推）

比方说，字符串 `"aeiou"` 和 `"aaaaaaeiiiioou"` 都是 **美丽的** ，但是 `"uaeio"` ，`"aeoiu"` 和 `"aaaeeeooo"` **不是美丽的** 。

给你一个只包含英文元音字母的字符串 `word` ，请你返回 `word` 中 **最长美丽子字符串的长度** 。如果不存在这样的子字符串，请返回 `0` 。

**子字符串** 是字符串中一个连续的字符序列。

 

**示例 1：**

```
输入：word = "aeiaaioaaaaeiiiiouuuooaauuaeiu"
输出：13
解释：最长子字符串是 "aaaaeiiiiouuu" ，长度为 13 。
```

**示例 2：**

```
输入：word = "aeeeiiiioooauuuaeiou"
输出：5
解释：最长子字符串是 "aeiou" ，长度为 5 。
```

**示例 3：**

```
输入：word = "a"
输出：0
解释：没有美丽子字符串，所以返回 0 。
```

 

**提示：**

- `1 <= word.length <= 5 * 105`
- `word` 只包含字符 `'a'`，`'e'`，`'i'`，`'o'` 和 `'u'` 。

------

[Discussion](https://leetcode-cn.com/problems/longest-substring-of-all-vowels-in-order/comments/) | [Solution](https://leetcode-cn.com/problems/longest-substring-of-all-vowels-in-order/solution/)

**思路**

1、先把所有字符按照元音下标进行转换，转成数字方便比较

2、设定两个指针 **i** 和 **j**

3、 右指针不断自增的过程中，如果发现它转换后的数值和前一个数值的差等于 0 或者 1 则正确，否则，令 **i = j**，断开之前的串

4、如果 **i** 对应的字符不是 'a'，则 **i** 继续自增；当 **j** 对应的字符是 'u' 的时候，更新长度最大值为 **j - 1 + 1**

**题解**

```rust
impl Solution {
    fn index(c: char) -> i32 {
        match c {
            'a' => 1,
            'e' => 2,
            'i' => 3,
            'o' => 4,
            'u' => 5,
            _ => 0,
        }
    }
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let word = word.chars().collect::<Vec<_>>();
        let (mut i, mut j) = (0i32, -1i32);
        let mut result = 0;
        while j < word.len() as i32 - 1 {
            j += 1;
            if j - i + 1 == 1 {
            } else if Self::index(word[j as usize]) - Self::index(word[j as usize - 1]) == 0
                || Self::index(word[j as usize]) - Self::index(word[j as usize - 1]) == 1
            {
            } else {
                i = j;
            }
            if word[i as usize] != 'a' {
                i += 1;
            } else if word[j as usize] == 'u' {
                result = result.max(j - i + 1);
            }
        }
        result
    }
}
```

