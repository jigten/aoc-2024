import {readFileSync} from 'node:fs';

const contents = readFileSync('sample.txt', 'utf-8'); 
const byte_locs = [];

for (const line of contents.split("\n")) {
    const [x, y] = line.split(",");
    byte_locs.push([Number(x), Number(y)]);
}

const rows = 7;
const cols = 7;
const grid = [];

for (let i = 0; i < rows; i++) {
    const r = [];

    for (let j = 0; j < cols; j++) {
        r.push(".");
    }

    grid.push(r);
}

for (let b = 0; b < 12; b++) {
    const [x, y] = byte_locs[b];
    grid[y][x] = "#";
}

const q = [[0, 0, 0]];
let res = 1000000;
const seen = new Set(['0,0']);

while (q.length > 0) {
    let [x, y, s] = q.shift();

    if (x === rows - 1 && y === cols - 1) {
        res = Math.min(res, s);
        continue;
    }

    for (let [dx, dy] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) {
        const newX = x + dx;
        const newY = y + dy;
        
        if (0 <= newX && newX < rows && 0 <= newY && newY < cols && grid[newX][newY] !== "#" && !seen.has(`${newX},${newY}`)) {
            q.push([newX, newY, s + 1]);
            seen.add(`${newX},${newY}`);
        }
    }
}

console.log("Solution: ", res);