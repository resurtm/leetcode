class Solution:
    def findMaxAverage(self, nums: List[int], k: int) -> float:
        i, j = 0, k
        sm = mx = sum(nums[i:j])
        while j < len(nums):
            sm -= nums[i]
            sm += nums[j]

            if sm > mx:
                mx = sm
            i += 1
            j += 1
        return mx / k
