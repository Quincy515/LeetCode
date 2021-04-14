package main

type MyStack struct {
	queue1 []int
	queue2 []int
}

/** Initialize your data structure here. */
func Constructor() MyStack {
	return MyStack{queue1: []int{}, queue2: []int{}}
}

/** Push element x onto stack. */
func (this *MyStack) Push(x int) {
	this.queue2 = append(this.queue2, x)
	for len(this.queue1) > 0 {
		this.queue2 = append(this.queue2, this.queue1[0])
		this.queue1 = this.queue1[1:]
	}
	this.queue1, this.queue2 = this.queue2, this.queue1
}

/** Removes the element on top of the stack and returns that element. */
func (this *MyStack) Pop() int {
	v := this.queue1[0]
	this.queue1 = this.queue1[1:]
	return v
}

/** Get the top element. */
func (this *MyStack) Top() int {
	return this.queue1[0]
}

/** Returns whether the stack is empty. */
func (this *MyStack) Empty() bool {
	return len(this.queue1) == 0
}

/**
 * Your MyStack object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(x);
 * param_2 := obj.Pop();
 * param_3 := obj.Top();
 * param_4 := obj.Empty();
 */
