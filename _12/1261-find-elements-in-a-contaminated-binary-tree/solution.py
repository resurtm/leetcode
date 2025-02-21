class FindElements:
    def __init__(self, root: Optional[TreeNode]):
        if root is None:
            return
        root.val = 0
        nums = set()

        def traverse(node):
            nonlocal nums
            nums.add(node.val)
            if node.left is not None:
                node.left.val = 2 * node.val + 1
                traverse(node.left)
            if node.right is not None:
                node.right.val = 2 * node.val + 2
                traverse(node.right)

        traverse(root)
        # self.root = root
        self.nums = nums

    def find(self, target: int) -> bool:
        return target in self.nums

    def find_other(self, target: int) -> bool:
        def traverse(node):
            if node.val == target:
                return True
            if node.left is not None and traverse(node.left):
                return True
            if node.right is not None and traverse(node.right):
                return True
            return False

        return traverse(self.root)
