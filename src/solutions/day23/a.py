from collections import defaultdict

adj = defaultdict(set)
trips = []

with open('input.txt') as f:
    for connection in f.read().splitlines():
        a, b = connection.split("-")
        adj[a].add(b)
        adj[b].add(a)

        for conn in adj[a]:
            if conn in adj[b]:
                trips.append((a, b, conn))

res = 0
for (a, b, c) in trips:
    if a[0].startswith("t") or b[0].startswith("t") or c.startswith("t"):
        res += 1

print(f"Solution: {res}")