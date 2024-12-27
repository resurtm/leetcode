class Solution:
    def chalkReplacer(self, its: List[int], k: int) -> int:
        c = k % sum(its)
        for idx, it in enumerate(its):
            c -= it
            if c < 0:
                return idx
        return -1

