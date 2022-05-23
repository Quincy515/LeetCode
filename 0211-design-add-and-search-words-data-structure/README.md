## [211. 添加与搜索单词 - 数据结构设计](https://leetcode.cn/problems/design-add-and-search-words-data-structure/)

请你设计一个数据结构，支持 添加新单词 和 查找字符串是否与任何先前添加的字符串匹配 。

实现词典类 `WordDictionary` ：

- `WordDictionary()` 初始化词典对象
- `void addWord(word)` 将 `word` 添加到数据结构中，之后可以对它进行匹配
- `bool search(word)` 如果数据结构中存在字符串与 `word` 匹配，则返回 `true` ；否则，返回 `false` 。`word` 中可能包含一些 `'.'` ，每个 `.` 都可以表示任何一个字母。



**示例：**

```
输入：
["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
[[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
输出：
[null,null,null,null,false,true,true,true]

解释：
WordDictionary wordDictionary = new WordDictionary();
wordDictionary.addWord("bad");
wordDictionary.addWord("dad");
wordDictionary.addWord("mad");
wordDictionary.search("pad"); // 返回 False
wordDictionary.search("bad"); // 返回 True
wordDictionary.search(".ad"); // 返回 True
wordDictionary.search("b.."); // 返回 True
```



**提示：**

- `1 <= word.length <= 25`
- `addWord` 中的 `word` 由小写英文字母组成
- `search` 中的 `word` 由 '.' 或小写英文字母组成
- 最多调用 `104` 次 `addWord` 和 `search`

```rust
#[derive(Default)]
struct WordDictionary {
    is_ending: bool,
    children: std::collections::HashMap<char, Self>,
}

impl WordDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: String) {
        let mut curr = self;
        for c in word.chars() {
            curr = curr.children.entry(c).or_insert_with(Self::new);
        }
        curr.is_ending = true;
    }

    fn search(&self, word: String) -> bool {
        self.inner_match(&word.chars().collect::<Vec<char>>())
    }
    fn inner_match(&self, word: &[char]) -> bool {
        let mut curr = self;
        for i in 0..word.len() {
            let c = word[i];
            if c == '.' {
                for child in curr.children.values() {
                    if child.inner_match(&word[(i + 1)..]) {
                        return true;
                    }
                }
            }
            if let Some(node) = curr.children.get(&c) {
                curr = node;
            } else {
                return false;
            }
        }
        curr.is_ending
    }
}
```

使用数组

```rust
#[derive(Default)]
struct WordDictionary {
    is_ending: bool,
    children: [Option<Box<Self>>; 26],
}

impl WordDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: String) {
        let mut curr = self;
        for i in word.bytes().map(|b| (b - b'a') as usize) {
            curr = curr.children[i].get_or_insert_with(Default::default);
        }
        curr.is_ending = true;
    }

    fn search(&self, word: String) -> bool {
        self.dfs(&word.chars().collect::<Vec<char>>(), 0)
    }

    fn dfs(&self, word: &[char], index: usize) -> bool {
        let curr = self;

        if index == word.len() {
            return curr.is_ending;
        }
        // 遇到通配符
        if word[index] == '.' {
            // pattern[i] 可以变化成任意字符，尝试所有可能，只要遇到一个匹配成功就返回
            for i in 0..26 {
                if let Some(node) = &curr.children[i] {
                    if node.dfs(word, index + 1) {
                        return true;
                    }
                }
            }
            // 都没有匹配
            return false;
        } else if let Some(node) = &curr.children[(word[index] as u8 - b'a') as usize] {
            // 从 node.children[c] 节点开始匹配 pattern[i+1..]
            return node.dfs(word, index + 1);
        }
        false
    }
}
```

