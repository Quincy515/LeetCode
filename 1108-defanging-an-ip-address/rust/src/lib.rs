struct Solution;

impl Solution {
    pub fn defang_i_paddr1(address: String) -> String {
        address.replace('.', "[.]")
    }

    pub fn defang_i_paddr2(address: String) -> String {
        let mut result = String::new();
        for c in address.chars() {
            if c == '.' {
                result.push_str("[.]");
            } else {
                result.push(c);
            }
        }
        result
    }

    pub fn defang_i_paddr3(address: String) -> String {
        let origin = address.as_bytes();
        let n = origin.len();
        let new_n = n + 2 * 3;
        let mut new = vec![' '; new_n];
        let mut k = 0;
        for i in 0..n {
            if origin[i] != b'.' {
                new[k] = origin[i] as char;
                k += 1;
            } else {
                new[k] = '[';
                k += 1;
                new[k] = '.';
                k += 1;
                new[k] = ']';
                k += 1;
            }
        }
        new.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::defang_i_paddr1("1.1.1.1".to_owned()),
            "1[.]1[.]1[.]1".to_owned()
        );
        assert_eq!(
            Solution::defang_i_paddr2("255.100.50.0".to_owned()),
            "255[.]100[.]50[.]0".to_owned()
        );
        assert_eq!(
            Solution::defang_i_paddr3("255.100.50.0".to_owned()),
            "255[.]100[.]50[.]0".to_owned()
        );
    }
}
