import {readFile} from 'node:fs/promises';

(async () => {
    const contents = await readFile('input.txt', 'utf-8');
    const trips = [];
    const adj = {};

    for (const connection of contents.split("\n")) {
        const [a, b] = connection.split("-");

        if (!(a in adj)) adj[a] = new Set();
        if (!(b in adj)) adj[b] = new Set();

        adj[a].add(b);
        adj[b].add(a);

        for (const conn of adj[a]) {
            if (adj[b].has(conn)) {
                trips.push([a, b, conn]);
            }
        }
    }
    let res = 0;

    for (const [a, b, c] of trips) {
        if (a.startsWith("t") || b.startsWith("t") || c.startsWith("t")) {
            res += 1;
        }
    }
    
    console.log(`Solution: ${res}`);
})();