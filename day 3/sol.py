import string

alphabet = string.ascii_lowercase + string.ascii_uppercase

priority = lambda c: alphabet.index(c)+1

def line_score(line):
    N = len(line)
    comp1, comp2 = line[:N//2], line[N//2:]

    duplicates = []
    for c1 in comp1:
        if c1 in comp2:
            duplicates.append(c1)
            break
    # print(duplicates)
    return sum([ priority(c) for c in duplicates ])

def group_badge(group):
    sets = [ set(line) for line in group ]
    return ( list(sets[0].intersection(sets[1]).intersection(sets[2]))[0] )

lines = []
with open("input", "r") as f:
    for line in f:
        l = line.strip()
        if len(l)>0:
            lines.append(l)


T = 0
for line in lines:
    T += line_score(line)

N = len(lines)
badges = [ group_badge([ lines[i*3+j] for j in range(3) ]) for i in range(N//3) ]
T2 = sum([ priority(c) for c in badges ])

print(T, T2)

# ## tests

# t1 = """vJrwpWtwJgWrhcsFMMfFFhFp
# jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
# PmmdzqPrVvPwwTWBwg"""
# t2 = """wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
# ttgJtRGJQctTZtZT
# CrZsJsPPZsGzwwsLwLmpwMDw"""

# print(group_badge(t1.split("\n")),
#       group_badge(t2.split("\n")))
