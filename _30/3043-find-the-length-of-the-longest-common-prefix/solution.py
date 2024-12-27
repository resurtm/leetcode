class Solution:
    def longestCommonPrefix(self, arr1: List[int], arr2: List[int]) -> int:
        st = set()
        for it in arr1:
            st.update(self.prefixes(it))
        res = -1
        for it in arr2:
            st2 = st.intersection(self.prefixes(it))
            if len(st2) > 0:
                res = max(res, max(st2))
        return len(str(res)) if res != -1 else 0

    def prefixes(self, num: int) -> set[int]:
        st, res = str(num), set()
        for i in range(1, len(st) + 1):
            res.add(int(st[:i]))
        return res

