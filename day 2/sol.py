import numpy as np

# moves
m = ["R", "P", "S"] # m[i] beats m[i-1]

# move codebook
p1 = { "A":"R", "B":"P", "C":"S" }
p2 = { "X":"R", "Y":"P", "Z":"S" }

# from coded inputs
def p2_wins(p1c, p2c):
    m1i = m.index(p1[p1c])
    m2i = m.index(p2[p2c])
    return ( (m2i-1)%3 )==m1i

#move score
ms = {"R":1, "P":2, "S":3}

def score(p1c, p2c):
    s = ms[p2[p2c]]
    if p1[p1c] == p2[p2c]:
        return s+3
    if p2_wins(p1c, p2c):
        return s+6
    return s

offset = {"X":-1, "Y":0, "Z":1} # move one forward, beats move behind
outcome_score = {"X":0, "Y":3, "Z":6} # loss, draw, win
def score2(p1c, outcome):
    m1i = m.index(p1[p1c])
    m2i = (m1i + offset[outcome])%3

    return ms[m[m2i]] + outcome_score[outcome]

instructions = []
with open("input", "r") as f:
    for line in f:
        instructions.append( line.split() )

TS = sum(map(lambda ins: score(*ins), instructions))
TS2 = sum(map(lambda ins: score2(*ins), instructions))

print(TS, TS2)

