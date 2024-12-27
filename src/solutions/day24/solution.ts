import {readFileSync} from 'node:fs';

const input = readFileSync('input.txt', 'utf-8');

const formulas = new Map();

for (const line of input.split("\n\n")[1].split("\n")) {
    const [x, op, y, z] = line.replace(" -> ", " ").split(" ");
    formulas[z] = [op, x, y];
}

const verifyZ = (wire, num) => {
    if (!(wire in formulas)) return;
    const [op, x, y] = formulas[wire];

    if (op !== 'XOR') {
        return false;
    }

    if (num == 0) {
        return x === 'x00' && y === 'y00' || x === 'y00' && y === 'x00';
    }

    return verifyIntermediate(x, num) && verifyCarry(y, num) || verifyIntermediate(y, num) && verifyCarry(x, num);
}

const makeWire = (char, num) => {
    return `${char}${String(num).padStart(2, "0")}`
}

const verifyIntermediate = (wire, num) => {
    if (!(wire in formulas)) return;
    const [op, x, y] = formulas[wire];

    if (op !== 'XOR') {
        return false;
    }

    return x === makeWire("x", num) && y === makeWire("y", num) || x === makeWire("y", num) && y === makeWire("x", num);
}

const verifyCarry = (wire, num) => {
    if (!(wire in formulas)) return;
    const [op, x, y] = formulas[wire];

    if (num === 1) {
        if (op !== 'AND') return false;
        return x === 'x00' && y === 'y00' || x === 'y00' && y === 'x00';
    }

    if (op !== 'OR') return false;

    return verifyDirectCarry(x, num - 1) && verifyReCarry(y, num - 1) || verifyDirectCarry(y, num - 1) && verifyReCarry(x, num - 1)
}

const verifyDirectCarry = (wire, num) => {
    if (!(wire in formulas)) return;
    const [op, x, y] = formulas[wire];

    if (op !== 'AND') return false;
    return x === makeWire("x", num) && y === makeWire("y", num) || x === makeWire("y", num) && y === makeWire("x", num);
}

const verifyReCarry = (wire, num) => {
    if (!(wire in formulas)) return;
    const [op, x, y] = formulas[wire];

    if (op !== 'AND') return false;
    return verifyIntermediate(x, num) && verifyCarry(y, num) || verifyIntermediate(y, num) && verifyCarry(x, num)
}

const verify = (num) => {
    return verifyZ(makeWire('z', num), num);
}

const progress = () => {
    let i = 0;
    while (true) {
        if (!(verify(i))) break;
        i += 1;
    }
    return i;
}

const swaps = [];
for (let i = 0; i < 4; i++) {
    let baseline = progress();
    let found = false;
    for (let x in formulas) {
        for (let y in formulas) {
            if (x === y) continue;

            let xFor = formulas[x];
            let yFor = formulas[y];

            formulas[y] = xFor;
            formulas[x] = yFor;

            if (progress() > baseline) {
                found = true;
                swaps.push(x);
                swaps.push(y);
                break;
            }

            formulas[x] = xFor;
            formulas[y] = yFor;
        }

        if (found) {        
            break;
        }
    }
}

console.log(swaps.sort().toString())

// console.log("failed on i", i, verify(i));

// z12 <-> qdg

// const pp = (wire, depth = 0) => {
//     if (['x', 'y'].includes(wire[0])) {
//         return " ".repeat(depth) + wire;
//     }
//     const [op, x, y] = formulas[wire];
//     return " ".repeat(depth) + "(" + op + ")\n" + pp(x, depth + 1) + "\n" + pp(y, depth + 1);
// }

// console.log(pp("z02"));