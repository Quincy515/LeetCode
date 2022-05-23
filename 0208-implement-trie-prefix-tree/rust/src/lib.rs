mod trie_map;

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
