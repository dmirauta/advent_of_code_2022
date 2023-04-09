import string

alphabet = string.ascii_lowercase + string.ascii_uppercase

priority = lambda c: alphabet.index(c)+1

def line_score(line):
    line = line.strip()
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
    sets = [ set(line.strip()) for line in group ]
    return ( list(sets[0].intersection(sets[1]).intersection(sets[2]))[0] )

with open("input", "r") as f:
    lines = f.readlines()

# with open("test_input", "r") as f:
#     tlines = f.readlines()


T = 0
for line in lines:
    T += line_score(line)

N = len(lines)
badges = [ group_badge([ lines[i*3+j] for j in range(3) ]) for i in range(N//3) ]
T2 = sum([ priority(c) for c in badges ])

print("duplicate sum =", T)
print("badge sum =", T2)

# ## tests
# print(group_badge(tlines[:3]),
#       group_badge(tlines[3:])
