# [1876.长度为三且各字符不同的子字符串](https://leetcode-cn.com/problems/substrings-of-size-three-with-distinct-characters/description/)

如果一个字符串不含有任何重复字符，我们称这个字符串为 **好** 字符串。

给你一个字符串 `s` ，请你返回 `s` 中长度为 **3** 的 **好子字符串** 的数量。

注意，如果相同的好子字符串出现多次，每一次都应该被记入答案之中。

**子字符串** 是一个字符串中连续的字符序列。

 

**示例 1：**

```
输入：s = "xyzzaz"
输出：1
解释：总共有 4 个长度为 3 的子字符串："xyz"，"yzz"，"zza" 和 "zaz" 。
唯一的长度为 3 的好子字符串是 "xyz" 。
```

**示例 2：**

```
输入：s = "aababcabc"
输出：4
解释：总共有 7 个长度为 3 的子字符串："aab"，"aba"，"bab"，"abc"，"bca"，"cab" 和 "abc" 。
好子字符串包括 "abc"，"bca"，"cab" 和 "abc" 。
```

 

**提示：**

- `1 <= s.length <= 100`
- `s` 只包含小写英文字母。

------

[Discussion](https://leetcode-cn.com/problems/substrings-of-size-three-with-distinct-characters/comments/) | [Solution](https://leetcode-cn.com/problems/substrings-of-size-three-with-distinct-characters/solution/)

**思路**

1、设计两个指针 **i** 和 **j** 分别指向字符串的开头

2、右指针不断向右移动，移动过程中如果发现右指针指向的字符出现超过一次，则左指针右移，直到把这一次降下来

3、如果窗口 **[i, j]** 长度大于3，则左指针一直往右移，如果窗口等于 3 则计数器加一

**题解**

```rust
impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut hash = std::collections::HashMap::new();
        let (mut l, mut r) = (0i32, -1i32);
        let mut result = 0;
        while r < s.len() as i32 - 1 {
            r += 1;
            let count = hash.entry(s[r as usize]).or_insert(0);
            *count += 1;
            while hash.get(&s[r as usize]) > Some(&1) || r - l + 1 > 3 {
                let tmp = hash.entry(s[l as usize]).or_insert(0);
                *tmp -= 1;
                l += 1;
            }
            if r - l + 1 == 3 {
                result += 1;
            }
        }
        result
    }
}
```

```c++
class Solution {
public:
    int countGoodSubstrings(string s) {
        int hash[256];
        int l = 0, r = -1;
        int size = s.size();
        int ans = 0;
        memset(hash, 0, sizeof(hash));
        while (r < size - 1) {
            // 右指针右移一位
            ++r;
            hash[s[r]]++;
            // 如果右指针对应的元素个数 > 1，如果长度 > 3 则继续调整区间
            while (hash[s[r]] > 1 || r - l + 1 > 3) {
                --hash[s[l]];
                l++;
            }
            if (r - l + 1 == 3) {
                ++ans;
            }
        }
        return ans;
    }
};
```

