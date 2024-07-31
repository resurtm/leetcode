class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        max = []
        cur_max = None
        for i in range(len(prices) - 1, -1, -1):
            if cur_max is None or cur_max < prices[i]:
                max.append(prices[i])
                cur_max = prices[i]
            else:
                max.append(cur_max)
        max.reverse()
        res = None
        for i in range(0, len(prices)):
            diff = max[i] - prices[i]
            if diff > 0 and (res is None or diff > res):
                res = diff
        return 0 if res is None or res < 0 else res

