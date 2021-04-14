package main

type CQueue struct {
	in  []int
	out []int
}

func Constructor() CQueue {
	return CQueue{
		in:  []int{},
		out: []int{},
	}
}

func (this *CQueue) AppendTail(value int) {
	this.in = append(this.in, value)
}

func (this *CQueue) DeleteHead() int {
	this.inToOut()
	if len(this.out) == 0 {
		return 0
	}
	v := this.out[len(this.out)-1]
	this.out = this.out[:len(this.out)-1]
	return v
}

/**
 * Your CQueue object will be instantiated and called as such:
 * obj := Constructor();
 * obj.AppendTail(value);
 * param_2 := obj.DeleteHead();
 */

func (this *CQueue) inToOut() {
	if len(this.out) != 0 {
		return
	}
	for len(this.in) > 0 {
		this.out = append(this.out, this.in[len(this.in)-1])
		this.in = this.in[:len(this.in)-1]
	}
}
