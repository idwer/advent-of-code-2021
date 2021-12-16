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

    drawn_numbers = get_drawn_number(rows)

    round = 0
    index = 0
    cap = input_board_height
    done = False

    while round <= number_of_boards:
        print(draw_numbers_per_round(drawn_numbers, index, cap, done))

        round += 1
        index = cap
        cap += input_board_height + round

        if round is number_of_boards:
            done = True
            print(draw_numbers_per_round(drawn_numbers, index, cap, done))
            break

    return score


if __name__ == '__main__':
    print(f"What will your final score be if you choose that board? {solution('input')}")
