class Solution:
    def longestSquareStreak(self, inp: List[int]) -> int:
        ns = sorted(inp)
        st = set(inp)
        res = 0
        for x in ns:
            vl = x
            cur = 1
            while vl * vl in st:
                vl = vl * vl
                cur += 1
            res = max(res, cur)
        return res if res >= 2 else -1
