class MyCircularDeque:
    def __init__(self, k: int):
        self.size = k
        self.items = []

    def insertFront(self, value: int) -> bool:
        if len(self.items) == self.size:
            return False
        self.items.insert(0, value)
        return True

    def insertLast(self, value: int) -> bool:
        if len(self.items) == self.size:
            return False
        self.items.append(value)
        return True

    def deleteFront(self) -> bool:
        if len(self.items) == 0:
            return False
        self.items.pop(0)
        return True

    def deleteLast(self) -> bool:
        if len(self.items) == 0:
            return False
        self.items.pop(-1)
        return True

    def getFront(self) -> int:
        if len(self.items) == 0:
            return -1
        return self.items[0]

    def getRear(self) -> int:
        if len(self.items) == 0:
            return -1
        return self.items[-1]

    def isEmpty(self) -> bool:
        return len(self.items) == 0

    def isFull(self) -> bool:
        return len(self.items) == self.size
