struct TopVotedCandidate {
    wins: Vec<i32>,
    times: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut hash = std::collections::HashMap::new();
        let mut top_persion = -1;
        let mut wins = Vec::new();
        for person in persons.iter() {
            let count = hash.entry(person).or_insert(0);
            *count += 1;
            if top_persion == -1 || *count >= *hash.get(&top_persion).unwrap_or(&0) {
                top_persion = *person;
            }
            wins.push(top_persion);
        }
        Self { wins, times }
    }

    fn q(&self, t: i32) -> i32 {
        let (mut left, mut right) = (0, self.times.len());
        let mut result = 0;
        while left < right {
            let mid = left + (right - left) / 2;
            if self.times[mid] <= t {
                result = mid as i32;
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        self.wins[result as usize]
    }
}

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
