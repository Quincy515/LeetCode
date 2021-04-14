use std::collections::VecDeque;

struct MyStack {
  queue1: VecDeque<i32>,
  queue2: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
  /** Initialize your data structure here. */
  fn new() -> Self {
    Self {
      queue1: VecDeque::new(),
      queue2: VecDeque::new(),
    }
  }

  /** Push element x onto stack. */
  fn push(&mut self, x: i32) {
    self.queue2.push_back(x);
    while self.queue1.len() > 0 {
      self.queue2.push_back(self.queue1.pop_front().unwrap());
    }
    std::mem::swap(&mut self.queue1, &mut self.queue2);
  }

  /** Removes the element on top of the stack and returns that element. */
  fn pop(&mut self) -> i32 {
    self.queue1.pop_front().unwrap()
  }

  /** Get the top element. */
  fn top(&self) -> i32 {
    self.queue1[0]
  }

  /** Returns whether the stack is empty. */
  fn empty(&self) -> bool {
    self.queue1.len() == 0
  }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let mut my_stack = MyStack::new();
    my_stack.push(1);
    my_stack.push(2);

    assert_eq!(2, my_stack.top());
    assert_eq!(2, my_stack.pop());
    assert_eq!(false, my_stack.empty());
  }
}
