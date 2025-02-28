class Solution:
    def numSplits(self, s: str) -> int:
        l, r = dict(collections.Counter(s)), collections.defaultdict(int)
        lu, ru = set(l.keys()), set()
        res = 0
        for ch in s:
            l[ch] -= 1
            if l[ch] == 0:
                lu.remove(ch)
            r[ch] += 1
            ru.add(ch)
            if len(lu) == len(ru):
                res += 1
        return res
