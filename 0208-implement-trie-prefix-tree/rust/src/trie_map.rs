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
