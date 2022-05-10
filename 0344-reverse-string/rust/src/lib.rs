struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (n, mut i, mut j) = (s.len(), 0, s.len() - 1);
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
