class Solution:
    def constructFromPrePost(self, pr: List[int], po: List[int]) -> Optional[TreeNode]:
        res, st, j = None, list(), 0

        for pri in pr:
            if len(st) == 0:
                res = TreeNode(pri)
                st.append(res)
                continue

            node = TreeNode(pri)
            if st[-1].left is None:
                st[-1].left = node
            else:
                st[-1].right = node
            st.append(node)

            while len(st) > 0 and j < len(po) and st[-1].val == po[j]:
                st.pop()
                j += 1

        return res
