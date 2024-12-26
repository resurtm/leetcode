class Solution:
    def maxVowels(self, s: str, k: int) -> int:
        vowels = set(["a", "e", "i", "o", "u"])
        curr = s[:k]
        a, b = 0, 0
        for ch in curr:
            if ch in vowels:
                a += 1
            else:
                b += 1
        pos = k
        res = a
        while k < len(s):
            if s[k] in vowels:
                a += 1
            else:
                b += 1
            ch = curr[0]
            curr = curr[1:] + s[k]
            if ch in vowels:
                a -= 1
            else:
                b -= 1
            k += 1
            if res is None or res < a:
                res = a
        return res
