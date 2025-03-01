class Solution:
    def countMatchingSubarrays(self, nums: List[int], pattern: List[int]) -> int:
        res, acc = 0, []
        for idx in range(len(nums)):
            if idx < len(nums) - len(pattern):
                acc.append((idx, 0))
            nxt = []
            for it in acc:
                if it[1] == len(pattern):
                    res += 1
                    continue
                if (
                    pattern[it[1]] == 1
                    and nums[it[0] + 1] > nums[it[0]]
                    or pattern[it[1]] == 0
                    and nums[it[0] + 1] == nums[it[0]]
                    or pattern[it[1]] == -1
                    and nums[it[0] + 1] < nums[it[0]]
                ):
                    nxt.append((it[0] + 1, it[1] + 1))
            acc = nxt
        return res
