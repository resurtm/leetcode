class Solution:
    def smallestNumber(self, pattern: str) -> str:
        st, used = [], [False] * 9

        def backtrack(pos: int, nxt: int) -> str | None:
            nonlocal st, used
            if pos == len(pattern):
                return "".join([str(t) for t in st])
            if pattern[pos] == "D":
                nxtc = nxt - 1
                while nxtc != 0:
                    if not used[nxtc - 1]:
                        st.append(nxtc)
                        used[nxtc - 1] = True
                        if res := backtrack(pos + 1, nxtc):
                            return res
                        used[nxtc - 1] = False
                        st.pop()
                    nxtc -= 1
            if pattern[pos] == "I":
                nxtc = nxt + 1
                while nxtc != 10:
                    if not used[nxtc - 1]:
                        st.append(nxtc)
                        used[nxtc - 1] = True
                        if res := backtrack(pos + 1, nxtc):
                            return res
                        used[nxtc - 1] = False
                        st.pop()
                    nxtc += 1
            return None

        for nxtc in range(1, 10):
            st.append(nxtc)
            used[nxtc - 1] = True
            if res := backtrack(0, nxtc):
                return res
            used[nxtc - 1] = False
            st.pop()
        return ""
