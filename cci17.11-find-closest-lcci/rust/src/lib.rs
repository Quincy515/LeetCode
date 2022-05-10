struct Solution;

impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        let (mut w1ps, mut w2ps) = (vec![], vec![]);
        for (i, word) in words.iter().enumerate() {
            if *word == word1 {
                w1ps.push(i as i32);
            } else if *word == word2 {
                w2ps.push(i as i32);
            }
        }
        let (mut p1, mut p2) = (0, 0);
        let mut min_ret = i32::MAX;
        while p1 < w1ps.len() && p2 < w2ps.len() {
            let (pos1, pos2) = (w1ps[p1], w2ps[p2]);
            if pos1 > pos2 {
                if min_ret > pos1 - pos2 {
                    min_ret = pos1 - pos2;
                }
                p2 += 1;
            } else {
                if min_ret > pos2 - pos1 {
                    min_ret = pos2 - pos1;
                }
                p1 += 1;
            }
        }

        min_ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_closest(
                vec![
                    "I".to_owned(),
                    "am".to_owned(),
                    "a".to_owned(),
                    "student".to_owned(),
                    "from".to_owned(),
                    "a".to_owned(),
                    "university".to_owned(),
                    "in".to_owned(),
                    "a".to_owned(),
                    "city".to_owned()
                ],
                "a".to_owned(),
                "student".to_owned()
            ),
            1
        );
    }
}
