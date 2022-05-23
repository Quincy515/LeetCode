struct Solution;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::suggested_products(
                vec![
                    "mobile".to_owned(),
                    "mouse".to_owned(),
                    "moneypot".to_owned(),
                    "monitor".to_owned(),
                    "mousepad".to_owned()
                ],
                "mouse".to_owned()
            ),
            vec![
                vec![
                    "mobile".to_owned(),
                    "moneypot".to_owned(),
                    "monitor".to_owned()
                ],
                vec![
                    "mobile".to_owned(),
                    "moneypot".to_owned(),
                    "monitor".to_owned()
                ],
                vec!["mouse".to_owned(), "mousepad".to_owned()],
                vec!["mouse".to_owned(), "mousepad".to_owned()],
                vec!["mouse".to_owned(), "mousepad".to_owned()]
            ]
        );
        assert_eq!(
            Solution::suggested_products(vec!["havana".to_owned()], "havana".to_owned()),
            vec![
                vec!["havana".to_owned()],
                vec!["havana".to_owned()],
                vec!["havana".to_owned()],
                vec!["havana".to_owned()],
                vec!["havana".to_owned()],
                vec!["havana".to_owned()],
            ]
        );
        assert_eq!(
            Solution::suggested_products(
                vec![
                    "bags".to_owned(),
                    "baggage".to_owned(),
                    "banner".to_owned(),
                    "box".to_owned(),
                    "cloths".to_owned()
                ],
                "bags".to_owned(),
            ),
            vec![
                vec!["baggage".to_owned(), "bags".to_owned(), "banner".to_owned()],
                vec!["baggage".to_owned(), "bags".to_owned(), "banner".to_owned()],
                vec!["baggage".to_owned(), "bags".to_owned()],
                vec!["bags".to_owned()]
            ]
        );
    }
}
