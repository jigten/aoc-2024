import { readFileSync } from 'node:fs';
import Heap from 'heap-js';

const findStart = (maze) => {
    for (let r = 0; r < maze.length; r++) {
        for (let c = 0; c < maze[r].length; c++) {
            if (maze[r][c] === "S") {
                return [r, c];
            }
        }
    }
    return null;
};

const maze_str = readFileSync('sample.txt', 'utf-8');
const rows = maze_str.split('\n').filter(row => row.trim() !== '');
const maze = rows.map(r => r.trim().split(""));

const start = findStart(maze);
if (!start) {
    console.error("Error: Start point 'S' not found in the maze.");
    process.exit(1);
}
const [r, c] = start;
const pq = new Heap((a, b) => a[0] - b[0]);
pq.push([0, r, c, 0, 1, [[r, c]]]);

const seen = new Set();
seen.add(`${r},${c},0,1`);

const seats = new Set();

while (pq.size() > 0) {
    let [s, r, c, dr, dc] = pq.pop();

    if (maze[r][c] === "E") {
        console.log("Solution ", s);
        break;
    }

    for (let [newScore, newR, newC, newDr, newDc] of [
        [s + 1, r + dr, c + dc, dr, dc],
        [s + 1000, r, c, dc, -dr],
        [s + 1000, r, c, -dc, dr]
    ]) {
        if (0 <= newR && newR < maze.length && 0 <= newC && newC < maze[0].length) {
            if (maze[newR][newC] === "#") continue;

            const stateKey = `${newR},${newC},${newDr},${newDc}`;
            if (seen.has(stateKey)) continue;

            pq.push([newScore, newR, newC, newDr, newDc]);
            seen.add(stateKey);
        }
    }
}
