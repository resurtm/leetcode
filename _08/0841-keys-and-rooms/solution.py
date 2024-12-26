class Solution:
    def canVisitAllRooms(self, rooms: List[List[int]]) -> bool:
        self.rooms = rooms
        self.viz = set()
        self.traverse(0)

        return len(self.viz) == len(self.rooms)

    def traverse(self, cur: int):
        self.viz.add(cur)
        for room in self.rooms[cur]:
            if room in self.viz:
                continue
            self.traverse(room)

