with open("input", "r") as f:
    lines = f.readlines()
s = lines[0]

with open("test_input", "r") as f:
    tlines = f.readlines()

def solve(s, packet_size=4):
    for i in range(len(s)-packet_size+1):
        if len(set(s[i:i+packet_size]))==packet_size:
            return i+packet_size

# #part 1 test
# for line in tlines:
#     print(solve(line))

# print()
# for line in tlines:
#     print(solve(line, ps=14))

print("part 1")
print(solve(s))

print("part 2")
print(solve(s, packet_size=14))

