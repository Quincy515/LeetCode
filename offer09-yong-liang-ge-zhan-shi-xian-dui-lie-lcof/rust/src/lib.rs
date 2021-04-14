struct CQueue {
  stack_in: Vec<i32>,
  stack_out: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
  fn new() -> Self {
    Self {
      stack_in: Vec::new(),
      stack_out: Vec::new(),
    }
  }

  fn append_tail(&mut self, value: i32) {
    self.stack_in.push(value);
  }

  fn delete_head(&mut self) -> i32 {
    self.transferIfEmpty();
    match self.stack_out.pop() {
      Some(value) => return value,
      None => return -1,
    }
  }

  fn peek(&mut self) -> i32 {
    self.transferIfEmpty();
    *self.stack_out.last().unwrap()
  }

  fn empty(&self) -> bool {
    self.stack_in.is_empty() && self.stack_out.is_empty()
  }
  // 辅助函数
  fn transferIfEmpty(&mut self) {
    if self.stack_out.is_empty() {
      while !self.stack_in.is_empty() {
        self.stack_out.push(self.stack_in.pop().unwrap());
      }
    }
  }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let mut queue = CQueue::new();
    queue.append_tail(1);
    queue.append_tail(2);

    assert_eq!(1, queue.peek());
    assert_eq!(1, queue.delete_head());
    assert_eq!(2, queue.delete_head());
    assert_eq!(true, queue.empty());
  }
}
