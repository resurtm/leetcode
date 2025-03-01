class Solution:
    def reverseWords(self, s: str) -> str:
        acc = ""
        res = ""
        for ch in s:
            if ch == " ":
                if acc != "":
                    res = f"{acc} {res}"
                acc = ""
            if ch != " ":
                acc += ch
        if acc != "":
            res = f"{acc} {res}"
        return res[:-1]
