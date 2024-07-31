class Solution:
    def reverseVowels(self, s: str) -> str:
        s = list(s)
        vow = "aeiouAEIOU"
        i, j = 0, len(s) - 1
        while i < j:
            if s[i] not in vow:
                i += 1
            if s[j] not in vow:
                j -= 1
            if s[i] in vow and s[j] in vow:
                s[i], s[j] = s[j], s[i]
                i += 1
                j -= 1
        return "".join(s)
