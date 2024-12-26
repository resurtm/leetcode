# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return None

        curr = head
        prev = None
        while curr is not None:
            next = curr.next

            curr.next = prev
            prev = curr
            curr = next
        return prev

    def reverseList2(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return None
        items = []
        curr = head
        while curr is not None:
            items.append(curr.val)
            curr = curr.next
        items.reverse()
        curr = head = None
        for item in items:
            if head is None:
                curr = head = ListNode(item)
                continue
            next = ListNode(item)
            curr.next = next
            curr = next
        return head

