from collections import OrderedDict


class LRUCache:
    def __init__(self, capacity: int):
        self.cap = capacity
        self.data = dict()
        self.usages = OrderedDict()

    def get(self, key: int) -> int:
        if key not in self.data:
            return -1

        if key in self.usages:
            self.usages.pop(key)
        self.usages[key] = True

        return self.data[key]

    def put(self, key: int, value: int) -> None:
        self.data[key] = value

        if key in self.usages:
            self.usages.pop(key)
        self.usages[key] = True

        if len(self.data) > self.cap:
            to_del = self.usages.popitem(False)
            self.data.pop(to_del[0])
