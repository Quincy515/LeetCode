class Solution {
  public class ListNode {
    int val;
    ListNode next;

    ListNode(int x) {
      val = x;
    }
  }
  
  // Time: O(n), Space: O(1) 遍历两次
  public ListNode middleNode(ListNode head) {
    ListNode p = head;
    int len = 0;
    for (; p != null; p = p.next) ++len;
    p = head;
    for (int i = 0; i < len/2; ++i) p = p.next;
    return p;
  }
  // Time: O(n), Space: O(1) 遍历一次
  public ListNode getMiddleNodeOnePass(ListNode head) {
    ListNode fast = head, slow = head;
    while (fast != null && fast.next != null) {
      fast = fast.next.next;
      slow = slow.next;
    }
    return slow;
  }
}