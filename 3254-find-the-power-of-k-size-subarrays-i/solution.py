class Solution:
    def resultsArray(self, nums: List[int], k: int) -> List[int]:
        arr = nums[0:k]

        srt = self.check(arr)
        mx = max(arr)

        res = [mx if srt else -1]
        for p in range(k, len(nums)):
            # print(arr)
            x = arr.pop(0)
            arr.append(nums[p])
            srt = self.check(arr)
            mx = max(arr)
            res.append(mx if srt else -1)

        return res

    def check(self, arr: List[int]) -> bool:
        if len(arr) <= 1:
            return True
        for i in range(1, len(arr)):
            if arr[i - 1] + 1 != arr[i]:
                return False
        return True
