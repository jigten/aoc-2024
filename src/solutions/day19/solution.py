from collections import defaultdict
from functools import cache

with open('input.txt') as f:
    contents = f.read()

towels, patterns = contents.split("\n\n")

class TrieNode:
    def __init__(self):
        self.children = defaultdict(TrieNode)
        self.is_end = False

root = TrieNode()
for t in towels.split(","):
    node = root
    for c in t.strip():
        node = node.children[c]
    node.is_end = True

@cache
def possible(i, p, n):
    if i == len(p):
        return n.is_end

    if p[i] not in n.children:
        return False

    if p[i] in n.children and n.children[p[i]].is_end:
        return possible(i + 1, p, n.children[p[i]]) or possible(i + 1, p, root)
    else:
        return possible(i + 1, p, n.children[p[i]]) 


count = 0
for p in patterns.split("\n"):
    if possible(0, p, root):
        count += 1

print(f"Solution: {count}")