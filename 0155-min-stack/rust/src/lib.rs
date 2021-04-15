struct MinStack {
  stack: Vec<i32>,
  min: Vec<i32>,
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
      min: Vec::new(),
    }
  }

  fn push(&mut self, val: i32) {
    self.stack.push(val);
    if self.min.is_empty() || val <= self.get_min() {
      self.min.push(val)
    }
  }

  fn pop(&mut self) {
    if *self.stack.last_mut().unwrap() == self.get_min() {
      self.min.pop();
    }
    self.stack.pop();
  }

  fn top(&mut self) -> i32 {
    *self.stack.last_mut().unwrap()
  }

  fn get_min(&mut self) -> i32 {
    *self.min.last_mut().unwrap()
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
