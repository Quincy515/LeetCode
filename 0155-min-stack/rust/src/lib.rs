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
    MinStack {
      stack: Vec::new(),
      min_stack: Vec::new(),
    }
  }

  fn push(&mut self, val: i32) {
    self.stack.push(val);
    if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
      self.min_stack.push(val);
    }
  }

  fn pop(&mut self) {
    if self.stack.is_empty() { return; }
    if self.stack.pop().unwrap() == *self.min_stack.last().unwrap() {
      self.min_stack.pop();
    }
  }

  fn top(&self) -> i32 {
    return *self.stack.last().unwrap();
  }

  fn get_min(&self) -> i32 {
    return *self.min_stack.last().unwrap();
  }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(-3, min_stack.get_min());
    min_stack.pop();
    assert_eq!(0, min_stack.top());
    assert_eq!(-2, min_stack.get_min());
  }
}
