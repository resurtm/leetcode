import copy


class Solution:
    def setZeroes(self, m: List[List[int]]) -> None:
        m2 = copy.deepcopy(m)
        for i in range(len(m)):
            for j in range(len(m[0])):
                if m2[i][j] == 0:
                    for i2 in range(len(m)):
                        m[i2][j] = 0
                    for j2 in range(len(m[0])):
                        m[i][j2] = 0
