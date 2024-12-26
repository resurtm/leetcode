class Solution:
    def minBitFlips(self, start: int, goal: int) -> int:

        st = ("{0:b}".format(start))[::-1]

        go = ("{0:b}".format(goal))[::-1]
        res = 0
        for i in range(max(len(st), len(go))):
            ch1 = "0" if i >= len(st) else st[i]
            ch2 = "0" if i >= len(go) else go[i]
            if ch1 != ch2:
                res += 1
        return res

