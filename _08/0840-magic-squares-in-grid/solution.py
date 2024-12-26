class Solution:
    def check(self, m: List[List[int]]) -> bool:
        st = set(m[0] + m[1] + m[2])
        if len(st) != 9:
            return False
        if st != set([1, 2, 3, 4, 5, 6, 7, 8, 9]):
            return False
        sm = m[0][0] + m[0][1] + m[0][2]
        for i in range(0, 3):
            s = m[i][0] + m[i][1] + m[i][2]
            if s != sm:
                return False
        for j in range(0, 3):
            s = m[0][j] + m[1][j] + m[2][j]
            if s != sm:
                return False
        s = m[0][0] + m[1][1] + m[2][2]
        if s != sm:
            return False
        s = m[0][2] + m[1][1] + m[2][0]
        if s != sm:
            return False
        return True

    def numMagicSquaresInside(self, grid: List[List[int]]) -> int:
        w, h = len(grid), len(grid[0])
        if w < 3 or h < 3:
            return 0
        res = 0
        for i in range(1, w - 1):
            for j in range(1, h - 1):
                sub = [

                    [grid[i - 1][j - 1], grid[i - 1][j], grid[i - 1][j + 1]],
                    [grid[i][j - 1], grid[i][j], grid[i][j + 1]],
                    [grid[i + 1][j - 1], grid[i + 1][j], grid[i + 1][j + 1]],
                ]
                if self.check(sub):
                    res += 1
        return res

