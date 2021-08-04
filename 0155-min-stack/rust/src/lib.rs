// 首先定义好两个栈
struct MinStack {
  // 一个栈叫做 stack，负责栈的正常操作
  stack: Vec<i32>,
  // 一个栈叫做 min_stack，负责获取 stack 中的最小值，它等价于遍历 stack 中的所有元素，把升序的数字都删除掉，留下一个从栈底到栈顶降序的栈
  min_stack: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
  /** initialize your data structure here. */
  fn new() -> Self {
    // 在这个函数中初始化两个栈，传入的参数为空
    MinStack {
      // 初始化 stack
      stack: Vec::new(),
      // 初始化 min_stack
      min_stack: Vec::new(),
    }
  }

  // 普通栈：直接添加 push 进来的值
  // 辅助栈：每次 push 一个新元素的时候，将普通栈中最小的元素 push 进辅助栈
  fn push(&mut self, x: i32) {
    // 新添加的元素添加到 stack 中
    self.stack.push(x);
    // 判断 min_stack 是否为空，如果为空，直接同时把新添加的元素添加到 min_stack 中
    // 如果 min_stack 不为空
    if !self.min_stack.is_empty() {
      // 获取 min_stack 的栈顶元素
      let top = *self.min_stack.last().unwrap();
      // 只有新添加的元素不大于 min_stack 的栈顶元素，才允许添加到 min_stack 中，目的是为了让 min_stack 从栈底到栈顶是降序的
      if x <= top {
        self.min_stack.push(x);
      }
    } else {
      // 此时新添加的元素 x 小于 min_stack 的栈顶元素，加入到 min_stack 后依旧是从栈底到栈顶是降序的
      self.min_stack.push(x)
    }
  }

  // 普通栈：直接移除普通栈中的栈顶元素
  // 辅助栈：判断普通栈中刚刚移除的栈顶元素值是否和此时辅助栈中的栈顶元素相同
  // 如果是则将辅助栈中的栈顶元素移除，否则不执行操作
  // 这样的目的是为了让辅助栈中的栈顶元素始终是普通栈中的最小值
  fn pop(&mut self) {
    // 让 stack 执行正常的 pop 操作就行
    let pop = self.stack.pop().unwrap();
    // 由于 min_stack 中的所有元素都是来自于 stack 中，所以 stack 删除元素后，min_stack 也要考虑是否需要删除元素
    // 否则的话，min_stack 有可能保存一个 stack 中不存在的元素

    // 首先，获取 min_stack 的栈顶元素
    let top = *self.min_stack.last().unwrap();
    // 再判断 top 这个栈顶元素是否和 stack 移除的元素相等，如果相等，那么需要把 min_stack 中的栈顶元素一并移除
    if top == pop {
      // 移除 min_stack 的栈顶元素
      self.min_stack.pop();
    }
  }

  fn top(&self) -> i32 {
    // 返回 stack 的栈顶元素
    return *self.stack.last().unwrap();
  }

  fn get_min(&self) -> i32 {
    // 返回 min_stack 的栈顶元素
    return *self.min_stack.last().unwrap();
  }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
  /** initialize your data structure here. */
  fn new() -> Self {
    // 在这个函数中初始化两个栈，传入的参数为空
    MinStack {
      // 初始化 stack
      stack: Vec::new(),
      // 初始化 min_stack
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
