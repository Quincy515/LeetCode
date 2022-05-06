# [面试题 10.02. 变位词组](https://leetcode-cn.com/problems/group-anagrams-lcci/)

编写一种方法，对字符串数组进行排序，将所有变位词组合在一起。变位词是指字母相同，但排列不同的字符串。

**注意：**本题相对原题稍作修改

**示例:**

```
输入: ["eat", "tea", "tan", "ate", "nat", "bat"],
输出:
[
  ["ate","eat","tea"],
  ["nat","tan"],
  ["bat"]
]
```



**说明：**

- 所有输入均为小写字母。
- 不考虑答案输出的顺序。

**思路** 

1、定义一个字符串到字符串数组的哈希表映射（可以用 c++ 中的 `unordered_map<string, vector<string>>`）

2、对于每个字符串，按照某种规则排序（比如字典序排序），然后插入到哈希表中

3、遍历哈希表，将它展平成一个二维数组

**题解**

```rust
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut unordered_map = std::collections::HashMap::new();
        let mut result = Vec::new();
        for str in strs {
            let mut s = str.chars().collect::<Vec<char>>();
            s.sort_unstable();
            unordered_map
                .entry(s.iter().collect::<String>())
                .or_insert(Vec::new())
                .push(str);
        }
        for v in unordered_map.values() {
            result.push(v.to_vec());
        }
        result
    }
}
```

