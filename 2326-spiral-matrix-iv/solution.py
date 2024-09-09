# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def spiralMatrix(self, m: int, n: int, head: Optional[ListNode]) -> List[List[int]]:
        res = []
        for j in range(m):
            tmp = []
            for i in range(n):
                tmp.append(-1)
            res.append(tmp)

        y, x, d = 0, 0, "R"
        curr = head
        while curr is not None:
            res[y][x] = curr.val
            curr = curr.next
            match d:
                case "R":
                    if x + 1 == n or res[y][x + 1] != -1:
                        d = "D"
                case "D":
                    if y + 1 == m or res[y + 1][x] != -1:
                        d = "L"
                case "L":
                    if x - 1 == -1 or res[y][x - 1] != -1:
                        d = "U"
                case "U":
                    if y - 1 == -1 or res[y - 1][x] != -1:
                        d = "R"
            match d:
                case "R":
                    x += 1
                case "D":
                    y += 1
                case "L":
                    x -= 1
                case "U":
                    y -= 1
        return res
