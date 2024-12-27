import {readFileSync} from 'node:fs';

const mix = (secret, value) => BigInt(secret) ^ BigInt(value);
const prune = (secret) => BigInt(secret) % BigInt(16777216);
const generateNextSecret = (secret) => {
    let nextSecret = prune(mix(secret, BigInt(secret) * BigInt(64)));
    nextSecret = prune(mix(nextSecret, nextSecret / BigInt(32)));
    return prune(mix(nextSecret, nextSecret * BigInt(2048)));
};

const secrets = readFileSync('input.txt', 'utf-8')
    .split('\n')
    .filter(line => line.trim() !== '')
    .map(BigInt);

const seqToBananas = new Map();
for (const secret of secrets) {
    const buyer = [secret % BigInt(10)]
    let nextSecret = secret;
    for (let i = 0; i < 2000; i++) {
        nextSecret = generateNextSecret(nextSecret);
        buyer.push(nextSecret % BigInt(10))
    }
    
    const seen = new Set();
    for (let i = 0; i < buyer.length - 4; i++) {
        const [a, b, c, d, e] = buyer.slice(i, i + 5);
        const key = `${b - a}${c - b}${d - c}${e - d}`;
        if (seen.has(key)) continue;
        seen.add(key);
        if (!(key in seqToBananas)) seqToBananas[key] = BigInt(0);
        seqToBananas[key] += BigInt(e);
    }
}

console.log("Solution:", Object.values(seqToBananas).reduce((max, value) => value > max ? value : max, BigInt(0)));