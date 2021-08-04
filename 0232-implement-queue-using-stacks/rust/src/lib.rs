// 首先定义好两个栈
struct MyQueue {
    // 一个栈叫做 stack_in，负责进栈操作，相当于队列 queue 中的入队操作
    stack_in: Vec<i32>,
    // 一个栈叫做 stack_out， 负责出栈操作，相当于队列 queue 中的出队操作
    stack_out: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        // 在这个函数中初始化两个栈，传入的参数为空
        Self {
            // 初始化 stack_in
            stack_in: Vec::new(),
            // 初始化 stack_out
            stack_out: Vec::new(),
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        // 新添加的元素添加到 stack_in 中
        self.stack_in.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        // 如果 stack_out 为空，首先需要将 stack_in 中的所有元素添加到 stack_out 中
        // 注意 stack_in 是栈，栈的性质是先进后出，后进先出，所以是不断的将 stack_in 中的栈顶元素添加到 stack_out 中
        if self.stack_out.is_empty() {
            // 通过 while 循环将 stack_in 中的所有元素都取出
            while !self.stack_in.is_empty(){
                // stack_out 不断的添加 stack_in 的栈顶元素
                self.stack_out.push(self.stack_in.pop().unwrap());
            }
        }
        // 此时，stack_in 已经为空，直接【移除】stack_out 的栈顶元素
        return self.stack_out.pop().unwrap();
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        // peek 和 pop 的区别在于是返回栈顶元素而非删除栈顶元素
        // 如果 stack_out 为空，首先需要将 stack_in 中的所有元素添加到 stack_out 中
        // 注意 stack_in 是栈，栈的性质是先进后出，后进先出，所以不断的将 stack_in 中的栈顶元素添加到 stack_out 中
        if self.stack_out.is_empty() {
            // 通过 while 循环将 stack_in 中的所有元素都取出
            while !self.stack_in.is_empty(){
                // stack_put 不断的添加 stack_in 的栈顶元素
                self.stack_out.push(self.stack_in.pop().unwrap())
            }
        }
        // peek 和 pop 的区别在于是返回栈顶元素而非删除栈顶元素
        // 此时，stack_in 已经为空，直接【返回】stack_out 的栈顶元素
        return self.stack_out[self.stack_out.len()-1]
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        // 队列是否为空，判断 stack_in 和 stack_put 是否同时不存在元素
        return self.stack_in.is_empty() && self.stack_out.is_empty();
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
