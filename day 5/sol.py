import re
from copy import copy

with open("input", "r") as f:
    lines = f.readlines()

with open("test_input", "r") as f:
    tlines = f.readlines()

def parse_input(lines, nstacks=9):
    crates = []
    for line in lines:
        if "[" in line:
            row = []
            for i in range(nstacks):
                chars = line[i*4:i*4+3]
                if chars[0] == "[":
                    row.append(chars[1])
                else:
                    row.append(None)
            crates.append(row)
        else:
            break

    stacks_ = [ [row[i] for row in crates] for i in range(nstacks) ]
    stacks = [ list(reversed(col)) for col in stacks_ ]

    for row in stacks:
        while row[-1] is None:
            row.pop()

    moves = []

    for line in lines:
        if "move" in line:
            moves.append( list(map(int, re.findall(r"move (\d+) from (\d+) to (\d+)", line)[0] )))

    return stacks, moves

def crate_mover9000(stacks, moves):
    for q, s1, s2 in moves:
        for i in range(q):
            c = stacks[s1-1].pop()
            stacks[s2-1].append(c)

def crate_mover9001(stacks, moves):
    for q, s1, s2 in moves:
        ministack = []
        for i in range(q):
            c = stacks[s1-1].pop()
            ministack.append(c)
        for i in range(q):
            c = ministack.pop()
            stacks[s2-1].append(c)

def solve(lines, nstacks=9, sim=crate_mover9001):
    stacks, moves = parse_input(lines, nstacks)
    sim(stacks, moves)
    return "".join([stack[-1] for stack in stacks])


# print(solve(tlines, 3, sim=simulate))
# print(solve(tlines, 3, sim=simulate2))

print(solve(lines, 9, sim=crate_mover9000))
print(solve(lines, 9, sim=crate_mover9001))


