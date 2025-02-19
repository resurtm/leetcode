class Solution:
    def getHappyString(self, n: int, k: int) -> str:
        s, d, kk = "", 0, 0

        def backtrack():
            nonlocal s, d, kk
            if d == n:
                kk += 1
                return s if kk == k else ""
            for ch in ["a", "b", "c"]:
                if len(s) > 0 and s[-1] == ch:
                    continue
                s = s + ch
                d += 1
                if r := backtrack():
                    return r
                s = s[:-1]
                d -= 1
            return ""

        return backtrack()
