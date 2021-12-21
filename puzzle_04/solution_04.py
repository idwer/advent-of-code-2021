from Board import Board


input_board_height = 0


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
    score = 0
    input_board_height = 5
    boards = []

    rows = []

    with open(filename, 'r') as infile:
        rows = [line for line in infile.read().splitlines()]
        infile.close()

    # take whitespace into account
    number_of_boards = get_number_of_boards(rows, input_board_height + 1)
    board_data = rows[1:]

    # take whitespace into account
    boards = generate_list_of_boards(number_of_boards, board_data, input_board_height + 1)
    # print(boards[0].tuples[0])
    # print(boards[0].tuples[5])
    # print(boards[0].tuples[10])
    # print(boards[0].tuples[15])
    # print(boards[0].tuples[20])

    drawn_numbers = get_drawn_number(rows)

    print(f"number of boards: {number_of_boards}")
    print(f"drawn numbers: {drawn_numbers}")

    round = 0
    index = 0
    cap = input_board_height
    done = False

    # while round <= number_of_boards:
    while round < number_of_boards:
        numbers_per_round = draw_numbers_per_round(drawn_numbers, index, cap, done)

        print(f"Round {round + 1}: {numbers_per_round}")
        [boards[round].mark_number(number) for number in numbers_per_round]

        # for board in boards:
        #     # [board.mark_number(number) for number in numbers_per_round]
        #     # [board.mark_number(number, input_board_height) for number in numbers_per_round]
        #     for number in numbers_per_round:
        #         board.mark_number(number)
        #         # board.mark_number(number, input_board_height)
        #         if board.has_winning_column(input_board_height):
        #             # or board.has_winning_row(input_board_height):
        #             print("yay")
        #             # return
        #         if board.has_winning_row(input_board_height):
        #             print("omg")
        #             # return

        # print(boards[round].has_winning_column(input_board_height))

        round += 1
        index = cap
        cap += input_board_height + round

        # if round is number_of_boards:
        #     done = True
        #     return

    # for board in boards:
    #     for x in range(0, len(board.tuples), 5):
    #         print(board.get_winning_row(x, x + 5))

    # print(boards[0].has_winning_column(input_board_height))
    # print(boards[1].has_winning_column(input_board_height))
    # print(boards[2].has_winning_column(input_board_height))

    for board in boards:
        print(board.get_marked_numbers())

    return score


if __name__ == '__main__':
    print(f"What will your final score be if you choose that board? {solution('test_input')}")
    # print(f"What will your final score be if you choose that board? {solution('input')}")
