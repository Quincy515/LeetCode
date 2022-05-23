mod trie_array;

#[derive(Default)]
struct WordDictionary {
    is_ending: bool,
    children: std::collections::HashMap<char, Self>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
