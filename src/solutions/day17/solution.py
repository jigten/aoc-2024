with open('input.txt') as f:
    lines = f.readlines()

registers = []
prog = []
for l in lines:
    if l.startswith("Register"):
        registers.append(int(l.split(":")[1].strip()))

    if l.startswith("Program"):
        prog.extend([int(d) for d in l.split(":")[1].split(",")])

a, b, c = registers
def combo(operand):
    if 0 <= operand <= 3:
        return operand
    elif operand == 4:  # Ensure operand maps to a valid index
        return a
    elif operand == 5:
        return b
    elif operand == 6:
        return c
    else:
        raise ValueError(f"Invalid operand value: {operand}")

res = []
i = 0
while i < len(prog):
    opcode = prog[i]
    operand = prog[i + 1]
    
    match opcode:
        case 0:
            a = a >> combo(operand)
        case 1:
            b = b ^ operand
        case 2:
            b = combo(operand) % 8
        case 3:
            if a != 0:
                i = operand
                continue
        case 4:
            b = b ^ c
        case 5:
            res.append(combo(operand) % 8)
        case 6:
            b = a >> combo(operand)
        case 7:
            c = a >> combo(operand)
    i += 2


print("Solution: ", ",".join(map(str, res)))