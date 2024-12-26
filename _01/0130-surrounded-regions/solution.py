class Solution:
    def solve(self, board: List[List[str]]) -> None:
        self.board = board
        self.viz = set()
        self.w = len(board)
        self.h = len(board[0])
        for i in range(0, self.w):
            for j in range(0, self.h):
                if board[i][j] == "O" and (i, j) not in self.viz:
                    rem, nodes = self.traverse(i, j)
                    if rem:
                        for node in nodes:
                            board[node[0]][node[1]] = "X"

    def traverse(self, i: int, j: int) -> (bool, list[(int, int)]):
        rem, nodes = True, []
        q = [(i, j)]
        while len(q) > 0:
            curr = q.pop()
            nodes.append(curr)
            self.viz.add(curr)
            if curr[0] > 0:
                if (
                    self.board[curr[0] - 1][curr[1]] == "O"
                    and (curr[0] - 1, curr[1]) not in self.viz
                ):
                    q.append((curr[0] - 1, curr[1]))
            else:
                rem = False
            if curr[1] > 0:
                if (
                    self.board[curr[0]][curr[1] - 1] == "O"
                    and (curr[0], curr[1] - 1) not in self.viz
                ):
                    q.append((curr[0], curr[1] - 1))
            else:
                rem = False
            if curr[0] < self.w - 1:
                if (
                    self.board[curr[0] + 1][curr[1]] == "O"
                    and (curr[0] + 1, curr[1]) not in self.viz
                ):
                    q.append((curr[0] + 1, curr[1]))
            else:
                rem = False
            if curr[1] < self.h - 1:
                if (
                    self.board[curr[0]][curr[1] + 1] == "O"
                    and (curr[0], curr[1] + 1) not in self.viz
                ):
                    q.append((curr[0], curr[1] + 1))
            else:
                rem = False
        return rem, nodes
