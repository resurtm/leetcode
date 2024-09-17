class Solution:
    def uncommonFromSentences(self, s1: str, s2: str) -> List[str]:
        s1, s2 = s1.split(" "), s2.split(" ")
        res = []
        for w in s1:
            if s1.count(w) == 1 and s2.count(w) == 0:
                res.append(w)
        for w in s2:
            if s2.count(w) == 1 and s1.count(w) == 0:
                res.append(w)
        return res
