with open('input.txt') as f:
    lines = f.readlines()

prog = []
for l in lines:
    if l.startswith("Program"):
        prog.extend([int(d) for d in l.split(":")[1].split(",")])

print(prog)
def find(prog, ans):
    if prog == []: return ans

    for t in range(8):
        a = (ans << 3) + t
        b = a % 8
        b = b ^ 1
        c = a >> b
        b = b ^ 4
        b = b ^ c
        if b % 8 == prog[-1]:
            sub = find(prog[:-1], a)
            if sub is None: continue
            return sub

print(find(prog, 0))
        