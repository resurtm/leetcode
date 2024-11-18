class Solution:
    def decrypt(self, code: List[int], k: int) -> List[int]:
        res = [0 for _ in code]
        if k == 0:
            return res
        revd = False
        if k < 0:
            code.reverse()
            k = -k
            revd = True
        for i in range(len(code)):
            s = 0
            for j in range(i + 1, i + k + 1):
                s += code[j % len(code)]
            res[i] = s
        return list(reversed(res)) if revd else res
