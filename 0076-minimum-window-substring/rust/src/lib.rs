use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (s, t) = (
            s.chars().collect::<Vec<char>>(),
            t.chars().collect::<Vec<char>>(),
        );
        let (mut min_wsize, mut min_wstart, mut min_wend) = (i32::MAX, -1i32, -1i32);
        // 模式串、滑动窗口
        let (mut tmap, mut wmap) = (HashMap::new(), HashMap::new());
        for c in t {
            let mut count = 1;
            // *tmap.entry(c).or_insert(0) += 1;
            if tmap.contains_key(&c) {
                count += tmap.get(&c).unwrap();
            }
            tmap.insert(c, count);
        }
        let (n, mut l, mut r) = (s.len() as i32, 0i32, -1i32);
        while l < n && r < n {
            while !Self::rmatch(&wmap, &tmap) {
                r += 1;
                if r > n - 1 {
                    break;
                }
                let c = s[r as usize];
                if tmap.contains_key(&c) {
                    let mut count = 1;
                    if wmap.contains_key(&c) {
                        count += wmap[&c];
                    }
                    wmap.insert(c, count);
                }
            }
            if Self::rmatch(&wmap, &tmap) {
                if min_wsize > r - l + 1 {
                    min_wsize = r - l + 1;
                    min_wstart = l;
                    min_wend = r;
                }
                let c = s[l as usize];
                if tmap.contains_key(&c) {
                    let count = wmap[&c];
                    if count - 1 == 0 {
                        wmap.remove(&c);
                    } else {
                        wmap.insert(c, count - 1);
                    }
                }
                l += 1;
            }
        }

        if min_wstart == -1 {
            return "".to_string();
        }
        s[min_wstart as usize..min_wend as usize + 1]
            .iter()
            .collect::<String>()
    }

    fn rmatch(wmap: &HashMap<char, i32>, tmap: &HashMap<char, i32>) -> bool {
        for (c, count) in tmap {
            if !wmap.contains_key(c) {
                return false;
            }
            if wmap.get(c).unwrap() < count {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
        assert_eq!(
            Solution::min_window("a".to_string(), "a".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::min_window("a".to_string(), "aa".to_string()),
            "".to_string()
        );
    }
}
