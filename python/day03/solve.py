import fileinput
import re

def solve_p1(string):
    print(string)
    sum = 0
    for match in re.finditer(r'mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)', string):
        print(match.group(0))
        print(match.group(1), match.group(2))
        sum += int(match.group(1)) * int(match.group(2))
    return sum

def solve_p2(string):
    # Scan do()s and don't()s
    dodont = {}
    mul = {}
    for match in re.finditer(r'do\(\)', string):
        dodont[match.start()] = "do"
    for match in re.finditer(r'don\'t\(\)', string):
        dodont[match.start()] = "dont"
    for match in re.finditer(r'mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)', string):
        mul[match.start()] = int(match.group(1)) * int(match.group(2))
    sum = 0
    mul_enabled = True
    for i in range(len(string)):
        if dodont.__contains__(i):
            if dodont[i] == "do":
                mul_enabled = True
            elif dodont[i] == "dont":
                mul_enabled = False
        if mul.__contains__(i):
            if mul_enabled:
                sum += mul[i]
    return sum




def main():
    string = ""
    for line in fileinput.input():
        string += line
    print(f'Result for problem 1: {solve_p1(string)}')
    print(f'Result for problem 2: {solve_p2(string)}')

if __name__ == "__main__":
    main()
