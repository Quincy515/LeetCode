// Definition for singly-linked list.
public class ListNode {
  int val;
  ListNode next;

  ListNode() {}

  ListNode(int val) { this.val = val; }

  ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}

class Solution {
  // Time: O(n), Space: O(1)
  public ListNode removeElements(ListNode head, int val) {
    ListNode dummy = new ListNode(0);
    dummy.next = head;
    ListNode notEqual = dummy;

    while (notEqual.next != null) {
      if (notEqual.next.val == val) notEqual.next = notEqual.next.next;
      else notEqual = notEqual.next;
    }
    return dummy.next;
  }
}