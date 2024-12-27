from collections import deque

with open('input.txt') as f:
    byte_locs = [(int(l.rstrip().split(',')[0]), int(l.rstrip().split(',')[1])) for l in f.readlines()]

grid = [['.'] * 71 for _ in range(71)]

for (x, y) in byte_locs[:1024]:
    grid[y][x] = "#"

q = deque([(0, 0, 0)])
res = float("inf")
seen = {(0, 0)}
rows, cols = len(grid), len(grid[0])

while q:
    for _ in range(len(q)):
        x, y, s = q.popleft()

        if x == rows - 1 and y == cols - 1:
            res = min(res, s)
        
        for dx, dy in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            new_x, new_y = x + dx, y + dy
            if 0 <= new_x < rows and 0 <= new_y < cols and grid[new_x][new_y] != "#" and (new_x, new_y) not in seen:
                seen.add((new_x, new_y))
                q.append((new_x, new_y, s + 1))
    
print("Solution: ", res)
    
    

        
