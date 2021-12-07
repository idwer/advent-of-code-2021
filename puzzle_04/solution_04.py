def solution() -> int:
    score = 0

    random_numbers = []
    randon_boards = []

    with open('test_input', 'r') as infile:
    # with open('input', 'r') as infile:
        rows = [n for n in infile.read().splitlines()]

        random_numbers = rows[0]
        randon_boards = rows[2:]

        print(random_numbers)
        print(randon_boards)

        infile.close()

    return score


if __name__ == '__main__':
    print(f"What will your final score be if you choose that board? {solution()}")
