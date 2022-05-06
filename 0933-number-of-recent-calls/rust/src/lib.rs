struct RecentCounter {
    requests: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.requests.push(t);
        let t_3000 = t - 3000;
        let mut result = 0;
        for i in self.requests.iter() {
            if *i >= t_3000 && *i <= t {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
