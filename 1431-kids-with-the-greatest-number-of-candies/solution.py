class Solution:
    def kidsWithCandies(self, candies: list[int], extraCandies: int) -> list[bool]:
        max = None
        for candy in candies:
            if max is None or max < candy:
                max = candy
        res: list[bool] = []
        for candy in candies:
            res.append(candy + extraCandies >= max)
        return res
