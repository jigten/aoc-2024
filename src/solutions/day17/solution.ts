import {readFileSync} from "node:fs";

const contents = readFileSync("input.txt", 'utf-8');

const registers = [];
let program: number[];

for (const line of contents.split("\n")) {
    if (line.startsWith("Register")) {
        registers.push(Number(line.split(":")[1].trim()))
    }
    
    if (line.startsWith("Program")) {
        program = line.split(":")[1].trim().split(",").map(Number)
    }
}

const combo = (operand: number) => {
    if (0 <= operand && operand <= 3) {
        return operand;
    } else if (operand === 4) {
        return a;
    } else if (operand === 5) {
        return b;
    } else if (operand === 6) {
        return c;
    } else {
        throw Error(`invalid operand: ${operand}`);
    }
}

let res: number[] = [];
let i = 0;
let [a, b, c] = registers;

while (i < program.length) {
    let opcode = program[i];
    let operand = program[i + 1];

    switch (opcode) {
        case 0:
            a = a >> combo(operand);
            break;
        case 1:
            b = b ^ operand;
            break;
        case 2:
            b = combo(operand) % 8;
            break;
        case 3:
            if (a !== 0) {
                i = operand;
                continue;
            }
            break;
        case 4:
            b = b ^ c;
            break;
        case 5:
            res.push(combo(operand) % 8);
            break;
        case 6:
            b = a >> combo(operand);
            break;
        case 7:
            c = a >> combo(operand);
            break;
    }

    i += 2;
}

console.log("Result: ", res.join(","));