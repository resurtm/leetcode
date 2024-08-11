class Solution:
    def increasingTriplet(self, nums: List[int]) -> bool:

        f = s = float("inf")
        for num in nums:
            if num <= f:
                f = num
                # s = float('inf')
            elif num <= s and num > f:
                s = num
            elif num > s:
                return True
        return False

