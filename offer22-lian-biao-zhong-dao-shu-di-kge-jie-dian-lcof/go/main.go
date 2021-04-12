package main

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func getKthFromEnd(head *ListNode, k int) *ListNode {
	cnt := 0
	fast, slow := head, head
	for fast != nil {
		if cnt >= k {
			slow = slow.Next
		}
		cnt++
		fast = fast.Next
	}
	return slow
}
