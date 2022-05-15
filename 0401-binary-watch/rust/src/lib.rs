struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut result = Vec::new();
        for i in 0..12 {
            for j in 0..60 {
                if turned_on as u32 == (i as i32).count_ones() + (j as i32).count_ones() {
                    let mut s = String::new();
                    s.push_str(&(i.to_string()));
                    s.push(':');
                    if j < 10 {
                        s.push('0');
                    }
                    s.push_str(&(j.to_string()));
                    result.push(s);
                }
            }
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
            Solution::read_binary_watch(1),
            vec![
                "0:01".to_owned(),
                "0:02".to_owned(),
                "0:04".to_owned(),
                "0:08".to_owned(),
                "0:16".to_owned(),
                "0:32".to_owned(),
                "1:00".to_owned(),
                "2:00".to_owned(),
                "4:00".to_owned(),
                "8:00".to_owned()
            ]
        );
    }
}
