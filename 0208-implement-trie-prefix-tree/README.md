## [208.实现 Trie (前缀树)](https://leetcode.cn/problems/implement-trie-prefix-tree/description/)

**[Trie](https://baike.baidu.com/item/字典树/9825209?fr=aladdin)**（发音类似 "try"）或者说 **前缀树** 是一种树形数据结构，用于高效地存储和检索字符串数据集中的键。这一数据结构有相当多的应用情景，例如自动补完和拼写检查。

请你实现 Trie 类：

- `Trie()` 初始化前缀树对象。
- `void insert(String word)` 向前缀树中插入字符串 `word` 。
- `boolean search(String word)` 如果字符串 `word` 在前缀树中，返回 `true`（即，在检索之前已经插入）；否则，返回 `false` 。
- `boolean startsWith(String prefix)` 如果之前已经插入的字符串 `word` 的前缀之一为 `prefix` ，返回 `true` ；否则，返回 `false` 。

 

**示例：**

```
输入
["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
[[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
输出
[null, null, true, false, true, null, true]

解释
Trie trie = new Trie();
trie.insert("apple");
trie.search("apple");   // 返回 True
trie.search("app");     // 返回 False
trie.startsWith("app"); // 返回 True
trie.insert("app");
trie.search("app");     // 返回 True
```

 

**提示：**

- `1 <= word.length, prefix.length <= 2000`
- `word` 和 `prefix` 仅由小写英文字母组成
- `insert`、`search` 和 `startsWith` 调用次数 **总计** 不超过 `3 * 104` 次

------

[Discussion](https://leetcode.cn/problems/implement-trie-prefix-tree/comments/) | [Solution](https://leetcode.cn/problems/implement-trie-prefix-tree/solution/)

**数组模板代码**

```rust
#[derive(Default, Debug)]
struct Trie {
    is_ending: bool,
    children: [Option<Box<Trie>>; 26],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut curr = self;
        for i in word.chars().map(|c| (c as u8 - b'a') as usize) {
            curr = curr.children[i].get_or_insert_with(Default::default);
            // curr = curr.children[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        curr.is_ending = true;
    }

    fn search(&self, word: String) -> bool {
        let mut curr = self;
        for i in word.bytes().map(|b| (b - b'a') as usize) {
            match &curr.children[i] {
                Some(node) => curr = node,
                None => return false,
            }
        }
        curr.is_ending
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        for i in prefix.bytes().map(|b| (b - b'a') as usize) {
            match &curr.children[i] {
                Some(node) => curr = node,
                None => return false,
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        trie.insert("hello".to_string());
        trie.insert("she".to_string());
        println!("{:?}", trie);
        let r = trie.search("hello".to_string());
        println!("{}", r);
    }
}
```

**使用 HashMap**

```rust
use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Self>,
    is_ending: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut curr = self;
        for c in word.chars() {
            curr = curr.children.entry(c).or_insert_with(Self::new);
        }
        curr.is_ending = true
    }

    fn search(&self, word: String) -> bool {
        let mut curr = self;
        for c in word.chars() {
            if let Some(node) = curr.children.get(&c) {
                curr = node;
            } else {
                return false;
            }
        }
        curr.is_ending
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        for c in prefix.chars() {
            if let Some(node) = curr.children.get(&c) {
                curr = node;
            } else {
                return false;
            }
        }
        true
    }
}
```

