class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        ops = "+-*/"
        st = []
        for token in tokens:
            if token not in ops:
                st.append(int(token))
                continue
            a, b = st.pop(), st.pop()
            if token == "+":

                st.append(b + a)
            elif token == "-":
                st.append(b - a)
            elif token == "/":
                st.append(int(b / a))
            elif token == "*":
                st.append(b * a)
        return st[-1]

