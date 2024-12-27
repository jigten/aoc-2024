with open('input.txt') as f:
    schematics = f.read().split('\n\n')

keys, locks = [], []

for s in schematics:
    schema = s.splitlines()
    heights = []
    for col in range(len(schema[0])):
        cnt = 0
        for row in range(1, len(schema) - 1):
            if schema[row][col] == '#':
                cnt += 1
        heights.append(cnt)
       
    if all(c == '#' for c in schema[0]):
        locks.append(heights)
    else:
        keys.append(heights) 
        
res = 0
for lock in locks:
    for key in keys:
        for (l, k) in zip(lock, key):
            if l + k >= 6:
                break
        else:
            res += 1

print(f"Solution: {res}")