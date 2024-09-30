class CustomStack:
    def __init__(self, sz: int):
        self.sz = sz
        self.st = []

    def push(self, x: int) -> None:
        if len(self.st) < self.sz:
            self.st.append(x)

    def pop(self) -> int:
        return -1 if len(self.st) == 0 else self.st.pop(-1)

    def increment(self, k: int, val: int) -> None:

        for x in range(min(k, len(self.st))):
            self.st[x] += val
