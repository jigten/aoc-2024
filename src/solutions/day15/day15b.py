file_path = "input.txt"

# Read the file into a string
with open(file_path, "r", encoding="utf-8") as file:
    file_content = file.read()

grid, commands = file_content.split('\n\n')

expanded_grid = []
for row in grid.splitlines():
    curr = []
    for c in row:
        if c in ['#', '.']:
            curr.extend([c, c])
        if c == 'O':
            curr.extend(['[', ']'])
        if c == '@':
            curr.extend(['@', '.'])
    expanded_grid.append(curr)


for r in expanded_grid:
    print(*r, sep='\t')

commands = commands.replace("\n", "")
rows, cols = len(expanded_grid), len(expanded_grid[0])
 
for r in range(rows):
    for c in range(cols):
        if expanded_grid[r][c] == "@":
            break
    else:
        continue
    break

for char in commands:
    dr = {"v": 1, "^": -1}.get(char, 0)
    dc = {"<": -1, ">": 1}.get(char, 0)
    go = True
    targets = [(r, c)]
    for (nr, nc) in targets:
        new_r, new_c = nr + dr, nc + dc
        if (new_r, new_c) in targets: continue
        char = expanded_grid[new_r][new_c]
        if char == "#":
            go = False
            break
        elif char == "[":
            targets.append((new_r, new_c))
            targets.append((new_r, new_c + 1))
        elif char == "]":
            targets.append((new_r, new_c))
            targets.append((new_r, new_c - 1))
       
    if not go: continue
    copy = [list(row) for row in expanded_grid]
    for (tr, tc) in set(targets[1:]):
        expanded_grid[tr][tc] = "."
    for (tr, tc) in set(targets[1:]):
        expanded_grid[tr + dr][tc + dc] = copy[tr][tc]    
    
    expanded_grid[r][c] = "."
    r = r + dr
    c = c + dc
    expanded_grid[r][c] = "@"

print("The answer is ", sum([r * 100 + c for r, row in enumerate(expanded_grid) for c, val in enumerate(row) if val == "["]))
