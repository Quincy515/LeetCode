struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut s: Vec<char> = Vec::new();
        s.push(chars[0]);
        let mut count = 0;
        for c in chars.iter() {
            if *c == s[s.len() - 1] {
                count += 1;
            } else {
                if count > 1 {
                    s.push(count.to_string().chars().next().unwrap());
                }
                s.push(*c);
                count = 1;
            }
        }
        if count > 1 {
            s.append(&mut count.to_string().chars().collect::<Vec<char>>());
        }
        println!("{:?}", s);
        chars.clear();
        chars.append(&mut s);
        chars.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(Solution::compress(&mut chars), 6);
        chars = vec!['a'];
        assert_eq!(Solution::compress(&mut chars), 1);
        chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(Solution::compress(&mut chars), 4);
    }
}
