class Solution:
    def diagonalSort(self, mat: List[List[int]]) -> List[List[int]]:
        w, h = len(mat), len(mat[0])
        for i in range(0, w):
            items = []
            for k in range(0, min(w - i, h)):
                items.append(mat[i + k][k])
            items.sort()
            for k in range(0, min(w - i, h)):
                mat[i + k][k] = items[k]
        for j in range(1, h):
            items = []
            for k in range(0, min(w, h - j)):
                items.append(mat[k][j + k])
            items.sort()
            for k in range(0, min(w, h - j)):
                mat[k][j + k] = items[k]
        return mat
