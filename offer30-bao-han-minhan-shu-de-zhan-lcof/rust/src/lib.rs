struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if !self.min_stack.is_empty() {
            let top = *self.min_stack.last().unwrap();
            if x <= top {
                self.min_stack.push(x);
            }
        } else {
            self.min_stack.push(x);
        }
    }
    
    fn pop(&mut self) {
        let pop = self.stack.pop().unwrap();
        let top = *self.min_stack.last().unwrap();
        if top == pop {
            self.min_stack.pop();
        }
    }
    
    fn top(&self) -> i32 {
        return *self.stack.last().unwrap();
    }
    
    fn min(&self) -> i32 {
        return *self.min_stack.last().unwrap();
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.min();
 */
