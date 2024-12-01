import fileinput

def solve_p1(l1, l2):
    #Sort lists
    l1.sort()
    l2.sort()
    difflist = []
    for i in range(len(l1)):
        # Compute the difference between each sorted element
        difflist.append(abs(l1[i] - l2[i]))
    # Finally, return the sum
    return sum(difflist)

def solve_p2(l1, l2):
    similarity_score = 0
    # For each number in l1...
    for num_l1 in l1:
        occurences = 0
        # Compute the number of occurences of the number in l2
        for num_l2 in l2:
            if num_l2 == num_l1:
                occurences += 1
        # Add occurences times the number to the similarity score
        similarity_score += num_l1 * occurences
    return similarity_score

def parse_input():
    l1 = []
    l2 = []
    # Parse the input
    for line in fileinput.input():
        # Split each line...
        i1, i2 = line.split()
        # And fill the lists by adding the first number to the first list and
        # the second number to the second list
        l1.append(int(i1))
        l2.append(int(i2))
    return l1, l2



def main():
    l1, l2 = parse_input()
    print(f'Result for problem 1: {solve_p1(l1, l2)}')
    print(f'Result for problem 2: {solve_p2(l1, l2)}')

if __name__ == "__main__":
    main()
