with open('input.txt') as f:
    grid = [list(r.rstrip()) for r in f.readlines()]


sr, sc = 0, 0
er, ec = 0, 0
rows, cols = len(grid), len(grid[0])

for (x, r) in enumerate(grid):
    for (y, c) in enumerate(r):
        if c == 'S':
            sr, sc = x, y  
        
        if c == 'E':
            er, ec = x, y


dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]
dists = [[-1] * cols for _ in range(rows)]
dists[sr][sc] = 0
si, sj = sr, sc

while (si, sj) != (er, ec):
    for (dr, dc) in dirs:
        new_i, new_j = si + dr, sj + dc
        if 0 <= new_i < rows and 0 <= new_j < cols and grid[new_i][new_j] != '#' and dists[new_i][new_j] == -1:
            dists[new_i][new_j] = dists[si][sj] + 1
            si = new_i
            sj = new_j

# for r in dists:
#     print(*r, sep='\t')

res = 0
for i in range(rows):
    for j in range(cols):
        if dists[i][j] == -1: continue
        for (di, dj) in [(-2, 0), (-1, 1), (0, 2), (1, 1), (2, 0), (1, -1), (0, -2), (-1, -1)]:
            new_i, new_j = i + di, j + dj

            if 0 <= new_i < rows and 0 <= new_j < cols and dists[new_i][new_j] != -1 and dists[new_i][new_j] > dists[i][j]:
                if dists[new_i][new_j] - dists[i][j] - 2 >= 100:
                    res += 1

print(f"Solution: {res}")