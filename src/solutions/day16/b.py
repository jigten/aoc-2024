import heapq
from collections import deque

def find_index(maze, t):
    for i, r in enumerate(maze):
        for j, c in enumerate(r):
            if c == t:
                return (i, j)

with open('input.txt') as f:
    maze = [l for l in f.readlines()]

rows, cols = len(maze), len(maze[0])
r, c = find_index(maze, "S")

start_r, start_c = find_index(maze, "S")

pq = [(0, start_r, start_c, 0 , 1)]
best_scores = {(start_r, start_c, 0, 1): 0}
backtrack = {}
best_score = float("inf")
end_states = set()

while pq:
    s, r, c, dr, dc = heapq.heappop(pq)
    if s > best_scores.get((r, c, dr, c), float('inf')): continue
    if maze[r][c] == "E":
        if s > best_score:
            break
        best_score = s
        end_states.add((r, c, dr, dc))

    for new_s, new_r, new_c, new_dr, new_dc in [(s + 1, r + dr, c + dc, dr, dc), (s + 1000, r, c, dc, -dr), (s + 1000, r, c, -dc, dr)]:
        if 0 <= new_r < rows and 0 <= new_c < cols:
            if maze[new_r][new_c] == "#": continue
            lowest = best_scores.get((new_r, new_c, new_dr, new_dc), float('inf'))
            if new_s > lowest: continue
            if new_s < lowest:
                best_scores[(new_r, new_c, new_dr, new_dc)] = new_s
                backtrack[(new_r, new_c, new_dr, new_dc)] = set()
            backtrack[(new_r, new_c, new_dr, new_dc)].add((r, c, dr, dc))
            heapq.heappush(pq, (new_s, new_r, new_c, new_dr, new_dc))

states = deque(end_states)
seen = set(end_states)

while states:
    key = states.popleft()
    for last in backtrack.get(key, []):
        if last in seen: continue
        seen.add(last)
        states.append(last)
    
print(len({(r, c) for r, c, _, _ in seen}))

