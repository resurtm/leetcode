class Solution:
    def getLucky(self, s: str, k: int) -> int:
        s2 = ""
        for ch in s:
            s2 += str(ord(ch) - ord("a") + 1)
        its = []
        for ch in s2:
            its.append(int(ch))
        for i in range(k):
            x = str(sum(its))
            its = []
            for ch in x:
                its.append(int(ch))
        res = ""
        for it in its:
            res += str(it)
        return int(res)

