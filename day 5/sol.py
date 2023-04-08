import re
from copy import copy

with open("input5", "r") as f:
    lines = f.readlines()

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
    stacks = [ list(reversed(row)) for row in stacks_ ]

    for row in stacks:
        while row[-1] is None:
            row.pop()

    moves = []

    for line in lines:
        if "move" in line:
            moves.append( list(map(int, re.findall(r"move (\d+) from (\d+) to (\d+)", line)[0] )))

    return stacks, moves

def simulate(stacks, moves):
    for q, s1, s2 in moves:
        for i in range(q):
            c = stacks[s1-1].pop()
            stacks[s2-1].append(c)

def simulate2(stacks, moves):
    for q, s1, s2 in moves:
        ministack = []
        for i in range(q):
            c = stacks[s1-1].pop()
            ministack.append(c)
        for i in range(q):
            c = ministack.pop()
            stacks[s2-1].append(c)

def solve(lines, nstacks=9, sim=simulate2):
    stacks, moves = parse_input(lines, nstacks)
    sim(stacks, moves)
    return "".join([stack[-1] for stack in stacks])

test_input = """    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"""

print(solve(test_input.split("\n"), 3))

print(solve(lines, 9))

