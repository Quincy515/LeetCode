struct Solution;
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut dict = [0; 26];
        for c in tiles.chars() {
            dict[c as usize - 'A' as usize] += 1;
        }
        let mut count = 0;
        Solution::blacktrack(&mut dict, &mut count);
        count
    }
    fn blacktrack(dict: &mut [i32; 26], count: &mut i32) {
        for i in 0..dict.len() {
            if dict[i] > 0 {
                *count += 1;
                dict[i] -= 1;
                Solution::blacktrack(dict, count);
                dict[i] += 1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::num_tile_possibilities("AAB".to_string()), 8);
        assert_eq!(Solution::num_tile_possibilities("AAABBC".to_string()), 188);
        assert_eq!(Solution::num_tile_possibilities("V".to_string()), 1);
    }
}
