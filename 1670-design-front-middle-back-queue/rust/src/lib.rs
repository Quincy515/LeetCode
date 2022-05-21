struct FrontMiddleBackQueue {
    vec: std::collections::VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            vec: std::collections::VecDeque::new(),
        }
    }

    fn push_front(&mut self, val: i32) {
        self.vec.push_front(val);
    }

    fn push_middle(&mut self, val: i32) {
        let size = self.vec.len();
        self.vec.insert(size / 2, val);
    }

    fn push_back(&mut self, val: i32) {
        self.vec.push_back(val);
    }

    fn pop_front(&mut self) -> i32 {
        self.vec.pop_front().unwrap_or(-1)
    }

    fn pop_middle(&mut self) -> i32 {
        let mut size = self.vec.len();
        if size % 2 == 1 {
            size /= 2;
        } else {
            size = size / 2 - 1;
        }
        self.vec.remove(size).unwrap_or(-1)
    }

    fn pop_back(&mut self) -> i32 {
        self.vec.pop_back().unwrap_or(-1)
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
