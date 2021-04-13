
// Definition for singly-linked list.
function ListNode(val) {
  this.val = val;
  this.next = null;
}


/**
 * @param {ListNode} headA
 * @param {ListNode} headB
 * @return {ListNode}
 */
var getIntersectionNode = function (headA, headB) {
  let lenA = 0, lenB = 0;
  for (let p = headA; p; p = p.next, ++lenA);
  for (let p = headB; p; p = p.next, ++lenB);
  let p = headA, q = headB;
  if (lenA > lenB) {
    for (let i = 0; i < lenA - lenB; ++i) p = p.next;
  } else {
    for (let i = 0; i < lenB - lenA; ++i) q = q.next;
  }
  while (p !== q) {
    p = p.next;
    q = q.next;
  }
  return p;
};

function getIntersectionNodeWithoutLen(headA, headB) {
  if (!headA || !headB) {
    return null;
  }
  let p = headA, q = headB;
  while (p !== q) {
    p = p === null ? headB : p.next;
    q = q === null ? headA : q.next;
  }
  return p;
}