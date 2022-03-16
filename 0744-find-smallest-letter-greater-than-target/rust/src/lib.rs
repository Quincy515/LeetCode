struct Solution;
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let (mut left, mut right) = (0, letters.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if target < letters[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        letters[left % letters.len()]
    }

    pub fn next_greatest_letter2(letters: Vec<char>, target: char) -> char {
        let mut index = letters.partition_point(|&c| c <= target);

        if index == letters.len() {
            index = 0;
        }

        letters[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'a';
        assert_eq!(Solution::next_greatest_letter(letters, target), 'c');
    }

    #[test]
    fn it_works2() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'c';
        assert_eq!(Solution::next_greatest_letter2(letters, target), 'f');
    }

    #[test]
    fn it_works3() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'd';
        assert_eq!(Solution::next_greatest_letter(letters, target), 'f');
    }

    #[test]
    fn it_works4() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'g';
        assert_eq!(Solution::next_greatest_letter(letters, target), 'j');
    }
}
