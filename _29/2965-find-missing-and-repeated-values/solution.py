class Solution:
    def findMissingAndRepeatedValues(self, grid: List[List[int]]) -> List[int]:
        vis, a, b, m = set(), -1, -1, -1
        for items in grid:
            for item in items:
                if item in vis:
                    a = item
                vis.add(item)
                m = max(m, item)
        for item in range(1, m + 1):
            if item not in vis:
                b = item
                break
        if b == -1:
            b = m + 1
        return [a, b]
