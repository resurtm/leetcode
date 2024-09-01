class Solution:
    def construct2DArray(self, original: List[int], m: int, n: int) -> List[List[int]]:
        if len(original) == 0:
            return []
        if m * n != len(original):
            return []
        res = []
        row = []
        for item in original:

            row.append(item)
            if len(row) == n:
                res.append(row)

                row = []
        return res

