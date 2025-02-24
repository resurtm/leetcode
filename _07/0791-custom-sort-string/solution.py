class Solution:
    def customSortString(self, order: str, s: str) -> str:
        o = [29] * 28
        for i, ch in enumerate(order):
            o[ord(ch) - ord("a")] = i
        s = list(s)
        s.sort(key=lambda ch: o[ord(ch) - ord("a")])
        return "".join(s)
