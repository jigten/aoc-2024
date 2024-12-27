import heapq

def find_index(maze, t):
    for i, r in enumerate(maze):
        for j, c in enumerate(r):
            if c == t:
                return (i, j)

with open('sample.txt') as f:
    maze = [l for l in f.readlines()]

rows, cols = len(maze), len(maze[0])
r, c = find_index(maze, "S")

start_r, start_c = find_index(maze, "S")
result = float("inf")

pq = [(0, start_r, start_c, 0 , 1)]
seen = {(start_r, start_c, 0, 1)}

while pq:
    s, r, c, dr, dc = heapq.heappop(pq)
    seen.add((r, c, dr, dc))

    if maze[r][c] == "E":
        print("Solution: ", s)
        break

    for new_s, new_r, new_c, new_dr, new_dc in [(s + 1, r + dr, c + dc, dr, dc), (s + 1000, r, c, dc, -dr), (s + 1000, r, c, -dc, dr)]:
        if 0 <= new_r < rows and 0 <= new_c < cols:
            if maze[new_r][new_c] == "#": continue
            if (new_r, new_c, new_dr, new_dc) in seen: continue
            heapq.heappush(pq, (new_s, new_r, new_c, new_dr, new_dc))


    
    


