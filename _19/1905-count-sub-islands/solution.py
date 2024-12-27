class Solution:
    def countSubIslands(self, grid1: List[List[int]], grid2: List[List[int]]) -> int:
        # isls1 = self.findIslands(grid1)
        pos = self.findPos(grid1)

        isls2 = self.findIslands(grid2)
        cnt = 0
        for idx2 in range(0, len(isls2)):

            if isls2[idx2].issubset(pos):
                cnt += 1
        # for idx2 in range(0, len(isls2)):
        #     for idx1 in range(idx2, len(isls1)):
        #         if isls2[idx2].issubset(isls1[idx1]):

        #             cnt += 1
        return cnt


    def findPos(self, grid: List[List[int]]) -> set:
        w, h = len(grid), len(grid[0])
        viz = set()
        for i in range(w):
            for j in range(h):
                if grid[i][j] == 0:
                    continue
                viz.add((i, j))
        return viz

    def findIslands(self, grid: List[List[int]]) -> list[set]:

        w, h = len(grid), len(grid[0])
        viz = set()


        def traverse(m, n) -> set:
            nonlocal viz
            q = [(m, n)]
            res = set()
            res.add((m, n))
            while len(q) > 0:
                a, b = q.pop()
                dirs = [(a - 1, b), (a + 1, b), (a, b - 1), (a, b + 1)]
                for k, l in dirs:
                    if (
                        k < 0
                        or k >= w
                        or l < 0
                        or l >= h
                        or grid[k][l] == 0
                        or (k, l) in viz
                    ):
                        continue
                    viz.add((k, l))
                    res.add((k, l))
                    q.append((k, l))
            return res

        retv = []
        for i in range(w):
            for j in range(h):
                if grid[i][j] == 0 or (i, j) in viz:
                    continue
                retv.append(traverse(i, j))

        return retv

