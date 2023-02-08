from Board import Board


def get_number_of_boards(rows: list, block_size: int) -> int:
    return int(len(rows[1:]) / block_size)


def get_drawn_numbers(rows: list) -> list:
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


def solution(filename: str, squid_must_win: bool) -> int:
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

    drawn_numbers = get_drawn_numbers(rows)

    if not squid_must_win:
        for number in drawn_numbers:
            for board in boards:
                board.mark_number(number)
                if board.has_winning_row_or_column(True):
                    return number * board.get_sum_of_unmarked_numbers()
                if board.has_winning_row_or_column(False):
                    return number * board.get_sum_of_unmarked_numbers()

    if squid_must_win:
        boards = generate_list_of_boards(number_of_boards, board_data, 5 + 1)

        boards_in_winning_order = []

        for number in drawn_numbers:
            if len(boards) == len(boards_in_winning_order):
                break

            for i, board in enumerate(boards):
                board.mark_number(number)
                if not board.won and (board.has_winning_row_or_column(True) or board.has_winning_row_or_column(False)):
                    boards_in_winning_order.append(board)

            if len(boards) == len(boards_in_winning_order) and (board.has_winning_row_or_column(True) or board.has_winning_row_or_column(False)):
                return number * boards_in_winning_order[len(boards_in_winning_order) - 1].get_sum_of_unmarked_numbers()

    return -1


if __name__ == '__main__':
    print(f"What will your final score be if you choose that board? test_input, first part: {solution('test_input', False)} (should be 4512)")
    print(f"What will your final score be if you choose that board? input, first part: {solution('input', False)} (should be 39902)")
    print(f"What will your final score be if you choose that board? test_input, second part: {solution('test_input', True)} (should be 1924)")
    print(f"What will your final score be if you choose that board? input, second part: {solution('input', True)} (should be 26936)")
