from collections import defaultdict
import heapq


class Solution:
    def maximumSum(self, nums: List[int]) -> int:
        cache = defaultdict(list)
        res = -1
        for num in nums:
            s = sum([int(ch) for ch in str(num)])
            if len(cache[s]) < 2:
                heapq.heappush(cache[s], num)
            else:
                heapq.heappushpop(cache[s], num)
            if len(cache[s]) >= 2:
                res = max(res, sum(heapq.nlargest(2, cache[s])))
        return res
