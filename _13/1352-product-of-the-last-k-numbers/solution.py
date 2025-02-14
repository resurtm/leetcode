class ProductOfNumbers:
    def __init__(self):
        self.nums = [1]

    def add(self, num: int) -> None:
        if num == 0:
            self.nums = [1]
        else:
            self.nums.append(num * self.nums[-1])

    def getProduct(self, k: int) -> int:
        if len(self.nums) <= 1 or k >= len(self.nums):
            return 0
        return self.nums[-1] // self.nums[-k - 1]
