struct CustomStack {
    stack: Vec<i32>,
    size: usize,
    top: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(maxSize: i32) -> Self {
        Self {
            stack: vec![0; maxSize as usize],
            size: maxSize as usize,
            top: 0,
        }
    }

    fn push(&mut self, x: i32) {
        if self.top < self.size {
            self.stack[self.top] = x;
            self.top += 1;
        }
    }

    fn pop(&mut self) -> i32 {
        if self.top != 0 {
            self.top -= 1;
            return self.stack[self.top];
        }
        -1
    }

    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..(k as usize).min(self.top) {
            self.stack[i] += val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
