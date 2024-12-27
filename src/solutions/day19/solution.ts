import {readFileSync} from 'node:fs';
import {randomUUID} from 'node:crypto';

export function DefaultDict(defaultFactory) {
    this.defaultFactory = defaultFactory;
    this.dict = new Map();

    this.get = (key) => {
        if (!this.dict.has(key)) {
            this.dict.set(key, this.defaultFactory());
        }
        return this.dict.get(key);
    }

    this.set = (key, value) => {
        this.dict.set(key, value);
    }

    this.has = (key) => {
        return this.dict.has(key);
    }
}

export class TrieNode {
    children: any;
    id: string;
    is_end: boolean;
    constructor() {
        this.children = new DefaultDict(() => new TrieNode());
        this.is_end = false;
        this.id = randomUUID();
    }
}

const solve = () => {
    const input = readFileSync('sample.txt', 'utf-8');
    const [towels, patterns] = input.split('\n\n');
    
    const root = new TrieNode();
    
    for (const t of towels.split(',')) {
        let node = root;
        for (let c of t.trim()) {
            node = node.children.get(c);
        }
        node.is_end = true;
    }
    
    const possible = (i, p, node) => {
        if (i === p.length) {
            return node.is_end;
        }
    
        if (!node.children.has(p[i])) {
            return false;
        }
    
        if (node.children.get(p[i]).is_end) {
            return possible(i + 1, p, node.children.get(p[i])) || possible(i + 1, p, root);
        } else {
            return possible(i + 1, p, node.children.get(p[i])); 
        }
    };
    
    
    let res = 0;
    for (const p of patterns.split('\n')) {
        if (possible(0, p, root)) {
            res += 1;
        }
    }
    console.log(`Solution: ${res}`);
}
