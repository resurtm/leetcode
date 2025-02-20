class Solution:
    def findDifferentBinaryString(self, nums: List[str]) -> str:
        sz, nums = len(nums[0]), set(nums)

        def backtrack(acc):
            if sz == len(acc):
                return None if acc in nums else acc
            if x := backtrack(acc + "0"):
                return x
            if x := backtrack(acc + "1"):
                return x
            return None

        return backtrack("")
