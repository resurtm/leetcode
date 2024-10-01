from collections import defaultdict


class Solution:
    def canArrange(self, its: List[int], k: int) -> bool:
        fr = defaultdict(int)
        for it in its:
            x = ((it % k) + k) % k
            fr[x] += 1
        if fr[0] % 2 != 0:
            return False
        for i in range(1, (k // 2) + 1):
            if fr[i] != fr[k - i]:
                return False
        return True
