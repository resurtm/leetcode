class Solution:
    def rotate(self, m: list[list[int]]) -> None:
        s = len(m) - 1
        for x in range(len(m) // 2):
            for i in range(x, s - x):
                a, b, c, d = m[x][i], m[i][s - x], m[s - x][s - i], m[s - i][x]
                m[x][i], m[i][s - x], m[s - x][s - i], m[s - i][x] = d, a, b, c
