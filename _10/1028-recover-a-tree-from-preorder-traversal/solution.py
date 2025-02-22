class Solution:
    def recoverFromPreorder(self, trav: str) -> Optional[TreeNode]:
        trav, st = Solution.parse(trav), []
        while len(trav) > 0:
            it = trav.pop(0)
            if len(st) == 0 or st[-1][0] == it[0] - 1:
                node = TreeNode(it[1])
                if len(st) != 0:
                    if st[-1][1].left is None:
                        st[-1][1].left = node
                    else:
                        st[-1][1].right = node
                st.append((it[0], node))
            else:
                for _ in range(st[-1][0] - it[0] + 1):
                    st.pop()
                node = TreeNode(it[1])
                if st[-1][1].left is None:
                    st[-1][1].left = node
                else:
                    st[-1][1].right = node
                st.append((it[0], node))

        return st[0][1]

    def parse(trav: str):
        st, acc, lev, res = 1, "", 0, []
        for ch in trav:
            if st == 1 and ch == "-":
                res.append((lev, int(acc)))
                st, acc, lev = 0, "", 0
            elif st == 0 and ch != "-":
                st, acc, lev = 1, "", len(acc)
            acc += ch
        res.append((lev, int(acc)))
        return res
