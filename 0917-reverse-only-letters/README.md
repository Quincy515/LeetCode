# [917.仅仅反转字母](https://leetcode-cn.com/problems/reverse-only-letters/description/)



给你一个字符串 `s` ，根据下述规则反转字符串：

- 所有非英文字母保留在原有位置。
- 所有英文字母（小写或大写）位置反转。

返回反转后的 `s` *。* 



**示例 1：**

```
输入：s = "ab-cd"
输出："dc-ba"
```



**示例 2：**

```
输入：s = "a-bC-dEf-ghIj"
输出："j-Ih-gfE-dCba"
```



**示例 3：**

```
输入：s = "Test1ng-Leet=code-Q!"
输出："Qedo1ct-eeLg=ntse-T!"
```

 

**提示**

- `1 <= s.length <= 100`
- `s` 仅由 ASCII 值在范围 `[33, 122]` 的字符组成
- `s` 不含 `'\"'` 或 `'\\'`

------

[Discussion](https://leetcode-cn.com/problems/reverse-only-letters/comments/) | [Solution](https://leetcode-cn.com/problems/reverse-only-letters/solution/)

**原理**

1、设计两个指针 **i** 和 **j** 分别指向字符串的两端

2、如果 **i** 指向的字符不为字母，则一直往右走；如果 **j** 指向的字符不为字母，则一直往左走

3、确保 **i** 和 **j** 指向的都为字母，并且执行交换，反复迭代

**题解**

```rust
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let (mut left, mut right) = (0, s.len() - 1);
        let mut s = s.chars().collect::<Vec<char>>();
        for _ in 0..s.len() {
            if left >= right { break; }
            if s[left].is_ascii_alphabetic() && s[right].is_ascii_alphabetic() {
                s.swap(left, right);
                left += 1;
                right -= 1;
            }
            if !s[left].is_ascii_alphabetic() { left += 1; }
            if !s[right].is_ascii_alphabetic() { right -= 1; }
        }

        s.iter().collect::<String>()
    }
}
```

