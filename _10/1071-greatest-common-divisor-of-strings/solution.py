class Solution:
    def gcdOfStrings(self, str1: str, str2: str) -> str:
        res = [self.find(str1, str2), self.find(str2, str1)]
        res.sort(key=lambda x: len(x))
        return res[-1]

    def find(self, s1: str, s2: str) -> str:
        if len(s1) < len(s2):
            return ""
        res = ""
        for i in range(1, len(s2) + 1):
            if len(s1) % i != 0 or len(s2) % i != 0:
                continue
            t1 = len(s1) // i
            t2 = len(s2) // i
            sub = s2[0:i]
            if sub * t1 == s1 and sub * t2 == s2:
                res = sub
        return res
