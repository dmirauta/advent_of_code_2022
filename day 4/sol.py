def ranges2sets(lines):
    ranges = []
    for line in lines:
        line_ranges = []
        for srange in line.strip().split(","):
            a,b = map(int, srange.split("-"))
            line_ranges.append(set(range(a,b+1)))
        #print(line_ranges)
        ranges.append(line_ranges)
    return ranges

def solution(lines):
    ranges=ranges2sets(lines)

    T = 0
    for i,(r1, r2) in enumerate(ranges):
        if r1.issubset(r2) or r2.issubset(r1):
            T += 1
            #print(i)

    return T

def solution_p2(lines):
    ranges=ranges2sets(lines)

    T = 0
    for i,(r1, r2) in enumerate(ranges):
        print(r1,r2)
        if len(r1.intersection(r2))>0:
            T += 1
            # print(i)

    return T

with open("input4", "r") as f:
    lines = f.readlines()

T = solution(lines)
T2 = solution_p2(lines)

#test
test = """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""

Tt = solution(test.split("\n"))

print(T, Tt)
print(T2)
