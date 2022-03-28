struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let s = s.chars().collect::<Vec<char>>();
        Self::backtrack(&s, 0, 0, &mut vec![], &mut result);

        result
    }

    fn backtrack(s: &[char], k: i32, step: i32, path: &mut Vec<i32>, result: &mut Vec<String>) {
        if k == s.len() as i32 && step == 4 {
            let mut sb: Vec<char> = vec![];
            for i in 0..3 {
                sb.push(s[path[i] as usize]);
                sb.push('.');
            }
            sb.push(s[path[3] as usize]);
            result.push(sb.iter().collect::<String>());
            return;
        }
        if step > 4 {
            return;
        }
        if k == s.len() as i32 {
            return;
        }
        let mut val = 0;
        // 1 位数
        if k < s.len() as i32 {
            val = val * 10 + (s[k as usize] as u8 - b'0') as i32;
            path.push(val);
            Self::backtrack(s, k + 1, step + 1, path, result);
            path.pop();
        }
        if s[k as usize] == '0' {
            // 前导0不行
            return;
        }
        // 2 位数
        if k + 1 < s.len() as i32 {
            val = val * 10 + (s[k as usize + 1] as u8 - b'0') as i32;
            path.push(val);
            Self::backtrack(s, k + 2, step + 1, path, result);
            path.pop();
        }
        // 3 位数
        if k + 2 < s.len() as i32 {
            val = val * 10 + (s[k as usize + 2] as u8 - b'0') as i32;
            if val <= 255 {
                path.push(val);
                Self::backtrack(s, k + 3, step + 1, path, result);
                path.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_string()),
            vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()]
        );
    }
}
