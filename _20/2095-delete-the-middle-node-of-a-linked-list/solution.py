# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

import math


class Solution:
    def deleteMiddle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        s = 0
        curr = head
        while curr is not None:
            curr = curr.next
            s += 1
        m = math.ceil((s - 1) / 2)

        n = 0
        curr = res = head
        prev = None
        mid = None
        while curr is not None:
            if m == n:
                if prev is not None:
                    prev.next = curr.next
                curr = curr.next
            else:
                prev = curr
                curr = curr.next
            n += 1
        if prev is None:
            return None
        return res
