package main

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

// Time: O(n), Space: O(1)
func oddEvenList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil || head.Next.Next == nil {
		return head
	}
	evenHead := head.Next
	odd, even := head, evenHead

	for even != nil && even.Next != nil {
		odd.Next = even.Next
		odd = odd.Next
		even.Next = odd.Next
		even = even.Next
	}
	odd.Next = evenHead
	return head
}
