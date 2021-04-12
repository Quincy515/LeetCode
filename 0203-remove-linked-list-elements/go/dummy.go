package dummy

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func removeElements(head *ListNode, val int) *ListNode {
	dummy := new(ListNode) // 新建一个虚拟头节点
	dummy.Next = head      // 虚拟头节点指向head
	notEqual := dummy      // 新建一个游标
	for notEqual.Next != nil {
		if notEqual.Next.Val == val {
			notEqual.Next = notEqual.Next.Next
		} else {
			notEqual = notEqual.Next
		}
	}
	return dummy.Next
}
