import {readFileSync} from "node:fs";

const contents = readFileSync("input.txt", 'utf-8');
let program = contents.split("\n\n")[1].split(":")[1].trim().split(",").map((v) => Number(v));

const find = (program: number[], answer) => {
    if (program.length === 0) {
        return answer;
    }

    for (let t = 0; t <= 7; t++) {
        let a = answer << 3 | t;
        let b = a % 8;
        b = b ^ 1;
        let c = a >> b;
        b = b ^ 4;
        b = b ^ c;
        if (b % 8 === program[program.length - 1]) {
            let sub = find(program.slice(0, program.length - 1), a);
            if (!sub) continue;
            return sub;
        }
    }
};

console.log("Result: ", find(program, 0));