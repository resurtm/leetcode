class Solution:
    def captureForts(self, forts: List[int]) -> int:
        st, curr, res = 0, 0, 0
        for fort in forts:
            if fort == 1:
                if st == -1:
                    curr, res = 0, max(res, curr)
                if st == 1:
                    curr = 0
                st = 1
            elif fort == -1:
                if st == -1:
                    curr = 0
                if st == 1:
                    curr, res = 0, max(res, curr)
                st = -1
            elif fort == 0:
                if st != 0:
                    curr += 1
        return res
