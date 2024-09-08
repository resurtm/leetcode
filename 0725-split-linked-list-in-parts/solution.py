# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def splitListToParts(
        self, head: Optional[ListNode], k: int
    ) -> List[Optional[ListNode]]:
        szi = 0
        curr = head
        while curr is not None:
            szi += 1
            curr = curr.next
        sz = [szi // k] * k
        for i in range(szi % k):
            sz[i] += 1

        curr = head
        res = []

        ress, resi = None, None
        idx, szc = 0, 0

        while curr is not None:
            if ress is None:
                ress = resi = ListNode(curr.val)
            else:
                tmp = ListNode(curr.val)
                resi.next = tmp
                resi = tmp

            szc += 1

            if sz[idx] == szc:
                res.append(ress)
                ress, resi = None, None
                idx += 1
                szc = 0
            curr = curr.next


        while len(res) < len(sz):
            res.append(None)
        return res

