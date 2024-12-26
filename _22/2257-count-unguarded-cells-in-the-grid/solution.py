class Solution:
    def countUnguarded(
        self, m: int, n: int, guards: List[List[int]], walls: List[List[int]]
    ) -> int:
        bl = ({(x, y) for x, y in guards}).union({(x, y) for x, y in walls})
        fi = bl.copy()
        for gx, gy in guards:
            for x in range(gx + 1, m):
                if (x, gy) in bl:
                    break
                fi.add((x, gy))
            for x in range(gx - 1, -1, -1):
                if (x, gy) in bl:
                    break
                fi.add((x, gy))
            for y in range(gy + 1, n):
                if (gx, y) in bl:
                    break
                fi.add((gx, y))
            for y in range(gy - 1, -1, -1):
                if (gx, y) in bl:
                    break
                fi.add((gx, y))
        return m * n - len(fi)
