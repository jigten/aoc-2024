from collections import defaultdict

adj = defaultdict(set)

with open('input.txt') as f:
    for connection in f.read().splitlines():
        a, b = connection.split("-")
        adj[a].add(b)
        adj[b].add(a)

sets = set()

def get_connected_nodes(n, grp):
    key = tuple(sorted(grp))  
    if key in sets: return
    sets.add(key)

    for conn in adj[n]:
        if conn in grp: continue
        if not all(conn in adj[query] for query in grp): continue
        grp.add(conn)
        get_connected_nodes(conn, grp)
        grp.remove(conn)


for n in adj:
    get_connected_nodes(n, {n})

largest = max(sets, key=len)
print(",".join(sorted(largest)))