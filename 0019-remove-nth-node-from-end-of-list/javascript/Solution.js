
// Definition for singly-linked list.
function ListNode(val, next) {
  this.val = (val === undefined ? 0 : val)
  this.next = (next === undefined ? null : next)
}

/**
 * @param {ListNode} head
 * @param {number} n
 * @return {ListNode}
 */
var removeNthFromEnd = function (head, n) {
  const dummy = new ListNode(0);
  dummy.next = head;
  let p = q = dummy;

  while (n > 0 && q.next) {
    q = q.next;
    n--;
  }

  if (n != 0) {
    return dummy.next;
  }

  while (q.next) {
    p = p.next;
    q = q.next;
  }
  p.next = p.next.next;
  return dummy.next;
};