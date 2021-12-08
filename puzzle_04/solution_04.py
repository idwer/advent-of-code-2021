from Board import Board


def solution() -> int:
    score = 0
    block_size = 6
    boards = []

    rows = []

    with open('test_input', 'r') as infile:
    # with open('input', 'r') as infile:
        rows = [n for n in infile.read().splitlines()]
        infile.close()

    number_of_boards = int(len(rows[1:]) / block_size)
    board_data = rows[1:]

    index = 0
    for _ in range(0, number_of_boards):
        param = board_data[index + 1:index + block_size]

        boards.append(Board(param))
        index += block_size

    return score


if __name__ == '__main__':
    print(f"What will your final score be if you choose that board? {solution()}")
