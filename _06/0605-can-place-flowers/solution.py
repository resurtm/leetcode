class Solution:
    def canPlaceFlowers(self, fl: list[int], n: int) -> bool:
        pos = 0
        tot = 0

        while pos < len(fl):
            st, en = pos - 1, pos + 1
            if st < 0:
                st = 0
            if en > len(fl) - 1:
                en = len(fl) - 1
            if 1 in fl[st : en + 1]:
                pos += 1
                continue
            fl[pos] = 1
            tot += 1

        return tot >= n
