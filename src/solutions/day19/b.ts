import {readFileSync} from 'node:fs';
import {TrieNode} from './solution';

const input = readFileSync('input.txt', 'utf-8');
const [towels, patterns] = input.split('\n\n');

const root = new TrieNode();

for (const t of towels.split(',')) {
    let node = root;
    for (let c of t.trim()) {
        node = node.children.get(c);
    }
    node.is_end = true;
}


for (const t of towels.split(',')) {
    let node = root;
    for (let c of t.trim()) {
        node = node.children.get(c);
    }
    node.is_end = true;
}

const memo = new Map();
const count = (i, p, node) => {
    const cacheKey = `${i}:${p}:${node.id}`;

    if (memo.has(cacheKey)) {
        return memo.get(cacheKey);
    }

    if (i === p.length) {
        const result = node.is_end ? 1 : 0;
        memo.set(cacheKey, result);
        return result;
    }

    if (!node.children.has(p[i])) {
        memo.set(cacheKey, 0);
        return 0;
    }

    const childNode = node.children.get(p[i]);

    let result;
    if (childNode.is_end) {
        result = count(i + 1, p, childNode) + count(i + 1, p, root);
    } else {
        result = count(i + 1, p, childNode);
    }

    memo.set(cacheKey, result);
    return result;
};

let res = 0;
for (const p of patterns.split('\n')) {
    res += count(0, p, root);
}

console.log(`Solution: ${res}`);