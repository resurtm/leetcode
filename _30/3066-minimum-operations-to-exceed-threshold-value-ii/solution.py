import heapq


class Solution:
    def minOperations(self, nums: List[int], k: int) -> int:
        heapq.heapify(nums)
        res = 0
        while nums[0] < k:
            a, b = heapq.heappop(nums), heapq.heappop(nums)
            heapq.heappush(nums, min(a, b) * 2 + max(a, b))
            res += 1
        return res
