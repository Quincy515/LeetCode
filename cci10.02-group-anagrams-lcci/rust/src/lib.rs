struct Solution;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::group_anagrams(vec![
                "eat".to_owned(),
                "tea".to_owned(),
                "tan".to_owned(),
                "ate".to_owned(),
                "nat".to_owned(),
                "bat".to_owned()
            ]),
            vec![
                vec!["ate".to_owned(), "eat".to_owned(), "tea".to_owned()],
                vec!["nat".to_owned(), "tan".to_owned()],
                vec!["bat".to_owned()]
            ]
        );
    }
}
