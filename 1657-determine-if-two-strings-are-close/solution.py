from collections import Counter


class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        c1 = dict(Counter(word1))
        c2 = dict(Counter(word2))
        if c1.keys() != c2.keys():

            return False
        return sorted(c1.values()) == sorted(c2.values())

