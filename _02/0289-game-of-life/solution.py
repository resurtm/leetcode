class Solution:
    def gameOfLife(self, board: list[list[int]]) -> None:
        w, h = len(board), len(board[0])

        res2 = []
        for i in range(w):
            lst = []
            for j in range(h):
                coords = [
                    (i - 1, j - 1),
                    (i - 1, j),
                    (i - 1, j + 1),
                    (i + 1, j - 1),
                    (i + 1, j),
                    (i + 1, j + 1),
                    (i, j - 1),
                    (i, j + 1),
                ]
                live, dead = 0, 0
                for x, y in coords:
                    if x == -1 or y == -1 or x == w or y == h:
                        continue
                    if board[x][y] == 1:
                        live += 1
                    else:
                        dead += 1
                res = board[i][j]
                if board[i][j] == 1:
                    if live < 2:
                        res = 0
                    elif live == 2 or live == 3:
                        res = 1
                    elif live > 3:
                        res = 0
                else:
                    if live == 3:
                        res = 1
                lst.append(res)
            res2.append(lst)

        for i in range(w):
            for j in range(h):
                board[i][j] = res2[i][j]
