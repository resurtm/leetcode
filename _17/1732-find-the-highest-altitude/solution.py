class Solution:
    def largestAltitude(self, gain: List[int]) -> int:
        pos = 0
        res = 0
        for g in gain:
            pos += g
            if pos > res:
                res = pos
        return res
