file_path = "sample.txt"

# Read the file into a string
with open(file_path, "r", encoding="utf-8") as file:
    file_content = file.read()

grid, commands = file_content.split('\n\n')

grid = [list(row) for row in grid.splitlines()]
commands = commands.replace("\n", "")
rows, cols = len(grid), len(grid[0])
 
for r in range(rows):
    for c in range(cols):
        if grid[r][c] == "@":
            break
    else:
        continue
    break

for char in commands:
    dr = {"v": 1, "^": -1}.get(char, 0)
    dc = {"<": -1, ">": 1}.get(char, 0)
    go = True
    targets = [(r, c)]
    while True:
        tr, tc = targets[-1]
        if grid[tr + dr][tc + dc] == "#":
            go = False
            break 
        elif grid[tr + dr][tc + dc] == "O":
            targets.append((tr + dr, tc + dc))
        else:
            go = True
            break

    if go:
        for (tr, tc) in targets[len(targets) - 1 : 0 : -1]:
            grid[tr][tc] = "."
            grid[tr + dr][tc + dc] = "O"
            
        grid[r][c] = "."
        r = r + dr
        c = c + dc
        grid[r][c] = "@"

print("The answer is ", sum([r * 100 + c for r, row in enumerate(grid) for c, val in enumerate(row) if val == "O"]))
