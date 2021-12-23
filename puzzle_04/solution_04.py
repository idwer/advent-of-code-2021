from Board import Board


def get_number_of_boards(rows: list, block_size: int) -> int:
    return int(len(rows[1:]) / block_size)


def get_drawn_number(rows: list) -> list:
    return [int(n) for n in rows[0].split(',')]


def generate_list_of_boards(number_of_boards: int, board_data: list, block_length: int) -> list:
    index = 0
    boards = []

    for _ in range(index, number_of_boards):
        param = board_data[index + 1:index + block_length]
        board = Board(param)

        boards.append(board)

        index += block_length

    return boards


def draw_numbers_per_round(drawn_numbers: list, index: int, cap: int, done: bool) -> list:
    if done:
        return drawn_numbers[index:]

    return drawn_numbers[index:cap]


def solution(filename: str) -> int:
    boards = []
    rows = []

    with open(filename, 'r') as infile:
        rows = [line for line in infile.read().splitlines()]
        infile.close()

    # take whitespace into account
    number_of_boards = get_number_of_boards(rows, 5 + 1)
    board_data = rows[1:]

    # take whitespace into account
    boards = generate_list_of_boards(number_of_boards, board_data, 5 + 1)

    drawn_numbers = get_drawn_number(rows)

    print(f"number of boards: {number_of_boards}")
    print(f"drawn numbers: {drawn_numbers}")

    for number in drawn_numbers:
        for board in boards:
            board.mark_number(number)
            if board.has_winning_row():
                return number * board.get_sum_of_unmarked_numbers()
            if board.has_winning_column():
                return number * board.get_sum_of_unmarked_numbers()
    return 0


if __name__ == '__main__':
    print(f"What will your final score be if you choose that board? {solution('test_input')}")
    # print(f"What will your final score be if you choose that board? {solution('input')}")
