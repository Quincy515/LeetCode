struct SortedStack {
  data: Vec<i32>,
  help: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SortedStack {
  fn new() -> Self {
    Self {
      data: Vec::new(),
      help: Vec::new(),
    }
  }

  fn push(&mut self, val: i32) {
    if !self.data.is_empty() && val > *self.data.last_mut().unwrap() {
      while !self.data.is_empty() && val > *self.data.last_mut().unwrap() {
        let temp = self.data.pop().unwrap();
        self.help.push(temp);
      }
      self.data.push(val);
      while !self.help.is_empty() {
        let temp = self.help.pop().unwrap();
        self.data.push(temp);
      }
    } else {
      self.data.push(val)
    }
  }

  fn pop(&mut self) {
    if !self.data.is_empty() {
      self.data.pop();
    }
  }

  fn peek(&mut self) -> i32 {
    if self.data.is_empty() {
      return -1;
    } else {
      return *self.data.last_mut().unwrap();
    }
  }

  fn is_empty(&self) -> bool {
    self.data.is_empty()
  }
}

/**
 * Your SortedStack object will be instantiated and called as such:
 * let obj = SortedStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.is_empty();
 */

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let mut sort_stack = SortedStack::new();
    sort_stack.push(1);
    sort_stack.push(2);
    assert_eq!(1, sort_stack.peek());
    sort_stack.pop();
    assert_eq!(2, sort_stack.peek());
    assert_eq!(false, sort_stack.is_empty());
    sort_stack.pop();
    assert_eq!(true, sort_stack.is_empty());
  }
}
