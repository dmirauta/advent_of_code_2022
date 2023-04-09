with open("input", "r") as f:
    lines = f.readlines()
s = lines[0]

with open("test_input", "r") as f:
    tlines = f.readlines()

def solve(s,
          ps=4 # package size
          ):
    for i in range(len(s)-ps+1):
        if len(set(s[i:i+ps]))==ps:
            return i+ps

# #part 1 test
# for line in tlines:
#     print(solve(line))

# print()
# for line in tlines:
#     print(solve(line, ps=14))

print("part 1")
print(solve(s))

print("part 2")
print(solve(s, ps=14))

