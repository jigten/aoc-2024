from collections import deque
from itertools import product
from functools import cache

with open('input.txt') as f:
    codes = f.read().splitlines()

numPad = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3'], [None, '0', 'A']]
dirPad = [[None, '^', 'A'], ['<', 'v', '>']]

def get_seqs(keypad):
    positions = {}
    for r in range(len(keypad)):
        for c in range(len(keypad[r])):
            if keypad[r][c] == None: continue
            positions[keypad[r][c]] = (r, c)

    seqs = {}
    for x in positions:
        for y in positions:
            if x == y:
                seqs[(x, y)] = ['A']
                continue

            optimal = float('inf')
            possibilities = []
            q = deque([(positions[x], "")])

            while q:
                (r, c), path = q.popleft()

                for (dr, dc, m) in [(r + 1, c, 'v'), (r - 1, c, '^'), (r, c - 1, '<'), (r, c + 1, '>')]:
                    if 0 <= dr < len(keypad) and 0 <= dc < len(keypad[dr]):
                        if keypad[dr][dc] == None: continue
                        if keypad[dr][dc] == y:
                            if optimal < len(path) + 1: break
                            optimal = len(path) + 1
                            possibilities.append(path + m + 'A')
                        else:
                            q.append(((dr, dc), path + m))
                else:
                    continue
                break
                
            seqs[(x, y)] = possibilities
    return seqs

def solve(code, seqs):
    options = [seqs[(a, b)] for a, b in zip('A' + code, code)]
    return ["".join(x) for x in product(*options)]

num_seqs = get_seqs(numPad)
dir_seqs = get_seqs(dirPad)
dir_lengths = {key: len(value[0]) for key, value in dir_seqs.items()}

@cache
def compute_length(x, y, depth = 2):
    if depth == 1:
        return dir_lengths[(x, y)]
    
    optimal = float('inf')
    for seq in dir_seqs[(x, y)]:
        length = 0
        for (i, j) in zip('A' + seq, seq):
            length += compute_length(i, j, depth - 1)
        optimal = min(optimal, length)
    return optimal
        

total = 0
for code in codes:
    inputs = solve(code, num_seqs)
    optimal = float('inf')

    for seq in inputs:
        length = 0
        for (i, j) in zip('A' + seq, seq):
            length += compute_length(i, j, 25)
        optimal = min(optimal, length)
    
    total += (optimal * int(code[:-1]))

print(f"Solution: {total}")
