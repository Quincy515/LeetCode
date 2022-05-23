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
        if word[index] == '.' {
            for i in 0..26 {
                if let Some(node) = &curr.children[i] {
                    if node.dfs(word, index + 1) {
                        return true;
                    }
                }
            }
            return false;
        } else if let Some(node) = &curr.children[(word[index] as u8 - b'a') as usize] {
            return node.dfs(word, index + 1);
        }
        false
    }
}
