with open('input.txt') as f:
    wires, gates = f.read().split("\n\n")

wire_vals = {}

for wire in wires.splitlines():
    name, val = wire.split(":")
    wire_vals[name] = int(val.strip())

z_count = 0
connections = []
for gate in gates.splitlines():
    conn, target = gate.split("->")
    curr_conn = conn.strip().split(" ")
    target = target.strip()
    if target.startswith("z"): z_count += 1
    curr_conn.append(target)
    connections.append(curr_conn)

z_left = z_count
while z_left > 0:
    for conn in connections:
        wire_a, gate, wire_b, target = conn

        if wire_a not in wire_vals or wire_b not in wire_vals:
            continue
            
        if target not in wire_vals and target.startswith("z"): z_left -= 1

        if gate == "AND":
            wire_vals[target] = wire_vals[wire_a] & wire_vals[wire_b]
        elif gate == "XOR":
            wire_vals[target] = wire_vals[wire_a] ^ wire_vals[wire_b]
        elif gate == "OR":
            wire_vals[target] = wire_vals[wire_a] | wire_vals[wire_b]
        else:
            raise ValueError(f"Unrecognized gate: {gate}")
    
    
z_buffer = [0] * z_count

def get_decimal(z_buffer):
    res = 0
    for i in range(len(z_buffer)):
        res += z_buffer[i] * (2 ** i)
    return res

for (wire, val) in wire_vals.items():
    if not wire.startswith("z"): continue
    idx = wire[1:].lstrip("0")
    if idx == "":
        z_buffer[0] = val
    else:
        z_buffer[int(idx)] = val
    
print(f"Solution: {get_decimal(z_buffer)}")