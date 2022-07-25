import sys


def get_rate(rows: list, line_width: int, gamma: bool) -> int:
    rate = 0

    for position in range(0, line_width):
        zeroes = 0
        ones = 0

        for row in rows:
            match row[position]:
                case '0':
                    zeroes += 1
                case '1':
                    ones += 1

        if position > 0:
            rate <<= 1

        if not gamma and ones < zeroes or gamma and ones > zeroes:
            rate |= 1

    return rate


def solution() -> int:
    with open('input', 'r') as infile:
        rows = [n for n in infile.read().splitlines()]
        infile.close()

        line_width = len(rows[0])

    return get_rate(rows, line_width, False) * get_rate(rows, line_width, True)


def solution_sample() -> int:
    with open('test_input', 'r') as infile:
        rows = [n for n in infile.read().splitlines()]
        infile.close()

        line_width = len(rows[0])

    return get_rate(rows, line_width, False) * get_rate(rows, line_width, True)


if __name__ == '__main__':
    print(f"What is the power consumption of the submarine? {solution_sample()} - using test_input")
    print(f"What is the power consumption of the submarine? {solution()}")