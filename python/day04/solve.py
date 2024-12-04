import fileinput
import re

def next_letter(l):
    word = ['X', 'M', 'A', 'S']
    return word[word.index(l) + 1]

def sign(num):
    if num < 0:
        return -1
    else:
        return 1
    
def search_xmas(matrix, i, j, l, dir):
    if l == 'S':
        return len(dir)
    if dir == []:
        return 0
    remaining_dir = []
    for d in dir:
        new_i = i + d[0]
        new_j = j + d[1]
        if new_i >= 0 and new_i < len(matrix) and new_j >= 0 and new_j < len(matrix[0]):
            let = matrix[new_i][new_j]
            if let == next_letter(l):
                new_dir = []
                if d[0] != 0:
                    new_dir.append(d[0] + 1 * sign(d[0]))
                else:
                    new_dir.append(0)
                if d[1] != 0:
                    new_dir.append(d[1] + 1 * sign(d[1]))
                else:
                    new_dir.append(0)
                remaining_dir.append(new_dir)
    return search_xmas(matrix, i, j, next_letter(l), remaining_dir)

def solve_p1(matrix):
    count = 0
    for i in range(len(matrix)):
        for j in range(len(matrix[i])):
            if matrix[i][j] == 'X':
                count += search_xmas(matrix, i, j, 'X', [
                    [0, -1],
                    [0, 1],
                    [-1, 0],
                    [1, 0],
                    [1, 1],
                    [-1, -1],
                    [1, -1],
                    [-1, 1]
                    ])
    return count

def solve_p2(matrix):
    count = 0
    for i in range(1, len(matrix) - 1):
        for j in range(1, len(matrix[i]) - 1):
            if matrix[i-1][j-1] == matrix[i+1][j+1]:
                continue
            if matrix[i][j] == 'A':
                num_s = 0
                num_m = 0
                for ind in [[-1, -1], [1, 1], [-1, 1], [1, -1]]:
                    new_i = i + ind[0]
                    new_j = j + ind[1]
                    let = matrix[new_i][new_j]
                    if let == 'S':
                        num_s += 1
                    if let == 'M':
                        num_m += 1
                if num_s == 2 and num_m == 2:
                    count += 1
    return count

def parse_input():
    matrix = []
    for line in fileinput.input():
        l = []
        for char in line:
            l.append(char)
        matrix.append(l)
    return matrix

def main():
    matrix = parse_input()
    print(f'Result for problem 1: {solve_p1(matrix)}')
    print(f'Result for problem 2: {solve_p2(matrix)}')


if __name__ == "__main__":
    main()
