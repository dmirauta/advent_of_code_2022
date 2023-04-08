with open("input6", "r") as f:
    lines = f.readlines()

s = lines[0]

test = """bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw""" # sols: 5,6,10,11

test2 = """mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw""" # sols: 19,23,23,29,26

def solve(s,
          ps=4 # package size
          ):
    for i in range(len(s)-ps+1):
        if len(set(s[i:i+ps]))==ps:
            return i+ps

for line in test.split("\n"):
    print(solve(line))

print()
print(solve(s))

print()
for line in test2.split("\n"):
    print(solve(line, ps=14))

print()
print(solve(s, ps=14))
