class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        self.b = board

        self.w = len(self.b)
        self.h = len(self.b[0])
        for i in range(self.w):
            for j in range(self.h):
                if self.b[i][j] == word[0]:
                    if self.traverse(i, j, word[1:], [[i, j]]):

                        return True
        return False

    def traverse(self, i: int, j: int, word: str, viz: list) -> bool:
        if len(word) == 0:
            return True
        dirs = [

            (i + 1, j),
            (i - 1, j),
            (i, j + 1),
            (i, j - 1),
        ]
        for x, y in dirs:
            if x < 0 or x >= self.w or y < 0 or y >= self.h or [x, y] in viz:

                continue
            if self.b[x][y] == word[0]:
                if self.traverse(x, y, word[1:], viz + [[x, y]]):
                    return True

