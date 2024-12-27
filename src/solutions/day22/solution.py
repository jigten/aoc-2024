def mix(secret, value):
    return secret ^ value

def prune(secret):
    return secret % 16777216

def generate_next_secret(secret):
    new_secret = prune(mix(secret, secret * 64))
    new_secret = prune(mix(new_secret, new_secret // 32))
    return prune(mix(new_secret, new_secret * 2048))

res = 0
with open("sample.txt") as f:
    for line in f.readlines():
        secret = int(line.rstrip())
        for _ in range(2000):
            secret = generate_next_secret(secret)
        
        res += secret

print(f"Solution: {res}")