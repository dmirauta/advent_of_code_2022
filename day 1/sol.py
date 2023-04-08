import numpy as np

def parse():
    elves_calories = [0]
    
    with open("input", "r") as f:
        elf=0
        for line in f:
            l = line.strip()
            if len(l) > 0:
                elves_calories[elf] += int(l)
            else:
                elves_calories.append(0)
                elf+=1

    return elves_calories

def part1(elves_calories):
    print("Part 1:\n", max(elves_calories), "\n")

def part2(elves_calories):
    eca = np.array(sorted(elves_calories))

    print("Part 2:")
    print(" top 3 ->", eca[-3:])
    print(" sum ->", eca[-3:].sum())


if __name__=="__main__":
    elves_calories = parse()

    part1(elves_calories)
    part2(elves_calories)
