import random


class RandomizedSet:

    def __init__(self):
        self.vals = dict()
        self.keys = list()

    def insert(self, val: int) -> bool:
        ret = val in self.vals
        if not ret:
            idx = len(self.keys)
            self.keys.append(val)

            self.vals[val] = idx
        return not ret

    def remove(self, val: int) -> bool:
        ret = val in self.vals
        if ret:
            vall = self.keys[-1]
            i, il = self.vals[val], self.vals[vall]
            self.keys[i], self.keys[il] = self.keys[il], self.keys[i]
            self.vals[vall] = i
            del self.vals[val]
            self.keys.pop()
        return ret

    def getRandom(self) -> int:
        return self.keys[random.randint(0, len(self.keys) - 1)]


# Your RandomizedSet object will be instantiated and called as such:
# obj = RandomizedSet()
# param_1 = obj.insert(val)
# param_2 = obj.remove(val)

# param_3 = obj.getRandom()

