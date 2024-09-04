class Solution:
    def robotSim(self, commands: List[int], obstacles: List[List[int]]) -> int:
        obst = set()
        for obstacle in obstacles:

            obst.add((obstacle[0], obstacle[1]))

        x, y, d = 0, 0, 0
        r = 0
        for command in commands:
            if command == -2:
                d -= 1
                if d < 0:
                    d = 3
                continue
            if command == -1:
                d += 1
                if d > 3:
                    d = 0
                continue
            if d == 0:
                for s in range(command):
                    y += 1
                    if (x, y) in obst:
                        y -= 1
                        break
            elif d == 1:

                for s in range(command):
                    x += 1
                    if (x, y) in obst:

                        x -= 1
                        break
            elif d == 2:
                for s in range(command):
                    y -= 1

                    if (x, y) in obst:
                        y += 1
                        break
            elif d == 3:
                for s in range(command):

                    x -= 1
                    if (x, y) in obst:
                        x += 1
                        break
            di = x * x + y * y
            if r < di:
                r = di
        return r

