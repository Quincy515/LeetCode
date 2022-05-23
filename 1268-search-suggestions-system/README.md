## [1268. 搜索推荐系统](https://leetcode.cn/problems/search-suggestions-system/)

给你一个产品数组 `products` 和一个字符串 `searchWord` ，`products` 数组中每个产品都是一个字符串。

请你设计一个推荐系统，在依次输入单词 `searchWord` 的每一个字母后，推荐 `products` 数组中前缀与 `searchWord` 相同的最多三个产品。如果前缀相同的可推荐产品超过三个，请按字典序返回最小的三个。

请你以二维列表的形式，返回在输入 `searchWord` 每个字母后相应的推荐产品的列表。



**示例 1：**

```
输入：products = ["mobile","mouse","moneypot","monitor","mousepad"], searchWord = "mouse"
输出：[
["mobile","moneypot","monitor"],
["mobile","moneypot","monitor"],
["mouse","mousepad"],
["mouse","mousepad"],
["mouse","mousepad"]
]
解释：按字典序排序后的产品列表是 ["mobile","moneypot","monitor","mouse","mousepad"]
输入 m 和 mo，由于所有产品的前缀都相同，所以系统返回字典序最小的三个产品 ["mobile","moneypot","monitor"]
输入 mou， mous 和 mouse 后系统都返回 ["mouse","mousepad"]
```

**示例 2：**

```
输入：products = ["havana"], searchWord = "havana"
输出：[["havana"],["havana"],["havana"],["havana"],["havana"],["havana"]]
```

**示例 3：**

```
输入：products = ["bags","baggage","banner","box","cloths"], searchWord = "bags"
输出：[["baggage","bags","banner"],["baggage","bags","banner"],["baggage","bags"],["bags"]]
```

**示例 4：**

```
输入：products = ["havana"], searchWord = "tatiana"
输出：[[],[],[],[],[],[],[]]
```



**提示：**

- `1 <= products.length <= 1000`
- `1 <= Σ products[i].length <= 2 * 10^4`
- `products[i]` 中所有的字符都是小写英文字母。
- `1 <= searchWord.length <= 1000`
- `searchWord` 中所有字符都是小写英文字母。

**题解**

```rust
#[derive(Default)]
struct Trie {
    word: Option<String>,
    children: std::collections::BTreeMap<char, Self>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut curr = self;
        for c in word.chars() {
            curr = curr.children.entry(c).or_default();
        }
        curr.word = Some(word);
    }

    fn find(&self, prefix: &str, limit: usize) -> Vec<String> {
        let mut curr = self;
        for c in prefix.chars() {
            match curr.children.get(&c) {
                None => return vec![],
                Some(node) => curr = node,
            }
        }

        let mut result = vec![];
        dfs(curr, &mut result, limit);
        result
    }
}

fn dfs(curr: &Trie, result: &mut Vec<String>, limit: usize) {
    if result.len() >= limit {
        return;
    }
    if let Some(word) = &curr.word {
        result.push(word.clone());
        if result.len() >= limit {
            return;
        }
    }

    for node in curr.children.values() {
        dfs(node, result, limit);
        if result.len() >= limit {
            return;
        }
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        products.into_iter().for_each(|word| trie.insert(word));

        let mut suggestions = vec![];
        for prefix_len in 1..=search_word.len() {
            let (prefix, _) = search_word.split_at(prefix_len);
            let words = trie.find(prefix, 3);
            suggestions.push(words);
        }

        suggestions
    }
}
```