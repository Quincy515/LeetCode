package main

type SortedStack struct {
	data []int
	help []int
}

func Constructor() SortedStack {
	return SortedStack{
		data: []int{},
		help: []int{},
	}
}

func (this *SortedStack) Push(val int) {
	if len(this.data) > 0 && val > this.data[len(this.data)-1] {
		for len(this.data) > 0 && val > this.data[len(this.data)-1] {
			temp := this.data[len(this.data)-1]
			this.data = this.data[:len(this.data)-1]
			this.help = append(this.help, temp)
		}
		this.data = append(this.data, val)
		for len(this.help) > 0 {
			temp := this.help[len(this.help)-1]
			this.help = this.help[:len(this.help)-1]
			this.data = append(this.data, temp)
		}
	} else {
		this.data = append(this.data, val)
	}
}

func (this *SortedStack) Pop() {
	if len(this.data) > 0 {
		this.data = this.data[:len(this.data)-1]
	}
}

func (this *SortedStack) Peek() int {
	if len(this.data) > 0 {
		return this.data[len(this.data)-1]
	} else {
		return -1
	}
}

func (this *SortedStack) IsEmpty() bool {
	return len(this.data) == 0
}

/**
 * Your SortedStack object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(val);
 * obj.Pop();
 * param_3 := obj.Peek();
 * param_4 := obj.IsEmpty();
 */
