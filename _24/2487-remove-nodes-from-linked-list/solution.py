class Solution:
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        # return Solution.removeNodesStack(head)
        return Solution.removeNodesReverse(head)

    def removeNodesStack(head: Optional[ListNode]) -> Optional[ListNode]:
        tmp = ListNode(float("inf"))
        st, curr = [tmp], head
        while curr is not None:
            while st[-1].val < curr.val:
                st.pop()
            st[-1].next = curr
            st.append(curr)
            curr = curr.next
        return tmp.next

    def removeNodesReverse(head: Optional[ListNode]) -> Optional[ListNode]:
        head = curr = Solution.reverseList(head)
        while curr is not None:
            nxt = curr.next
            while nxt is not None and nxt.val < curr.val:
                nxt = nxt.next
                curr.next = nxt
            curr = nxt
        return Solution.reverseList(head)

    def reverseList(head: Optional[ListNode]):
        curr, prev = head, None
        while curr is not None:
            curr.next, prev = prev, curr.next
            curr, prev = prev, curr
        return prev
