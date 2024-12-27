import {readFile} from 'node:fs/promises';

(async () => {
    const grid = (await readFile('input.txt', 'utf-8')).split("\n").map(r => r.split(""));
    const rows = grid.length, cols = grid[0].length;

    let sr: number, sc: number;
    let er: number, ec: number;

    for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
            if (grid[i][j] === "S") {
                sr = i;
                sc = j;
            }

            if (grid[i][j] === "E") {
                er = i;
                ec = j;
            }
        }
    }

    const dirs = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    const dists = new Array(rows).fill(0).map(() => new Array(cols).fill(-1));
    dists[sr][sc] = 0;
    let cr = sr, cc = sc;

    while (cr !== er || cc != ec) {
        for (const [dr, dc] of dirs) {
            let new_r = cr + dr, new_c = cc + dc;
            if (0 <= new_r && new_r < rows && 0 <= new_c && new_c < cols && grid[new_r][new_c] !== "#" && dists[new_r][new_c] === -1) {
                dists[new_r][new_c] = dists[cr][cc] + 1;
                cr = new_r;
                cc = new_c;
            }
        }
    }
    // for (const r of dists) {
    //     console.log(r.join('\t'))
    // }

    let res = 0;
    for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
            if (grid[i][j] === "#") {
                continue;
            }
            for (let radius = 2; radius <= 20; radius++) {
                for (let dr = 0; dr <= radius; dr++) {
                    let dc = radius - dr;
                    for (let new_coords of new Set([`${i+dr},${j+dc}`, `${i+dr},${j-dc}`, `${i-dr},${j+dc}`, `${i-dr},${j-dc}`])) {
                        let [new_i_str, new_j_str] = new_coords.split(",");
                        let new_i = Number(new_i_str), new_j = Number(new_j_str);
                        if (!(0 <= new_i && new_i < rows && 0 <= new_j && new_j < cols)) {
                            continue;
                        }
                        if (grid[new_i][new_j] === "#") {
                            continue;
                        }
                        if (dists[i][j] - dists[new_i][new_j] >= 100 + radius) {
                            res += 1;
                        }
                    }
                }
            }
        }
    }
    console.log("Solution: ", res);
})();
