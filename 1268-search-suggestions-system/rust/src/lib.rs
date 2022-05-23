struct Solution;

#[derive(Default)]
struct Trie {
    is_ending: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut curr = self;
        for i in word.bytes().map(|b| (b - b'a') as usize) {
            curr = curr.children[i].get_or_insert_with(Default::default);
        }
        curr.is_ending = true;
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
