import sys


def get_epsilon_rate(rows: list, line_width: int) -> int:
    epsilon_rate = 0

    zeroes = 0
    ones = 0

    for position in range(0, line_width):
        for _, line in enumerate(rows):
            match line[position]:
                case '0':
                    zeroes += 1
                case '1':
                    ones += 1

        if position > 0:
            epsilon_rate <<= 1

        if ones < zeroes:
            epsilon_rate |= 1

        ones = 0
        zeroes = 0

    return epsilon_rate


def get_gamma_rate(rows: list, line_width: int) -> int:
    gamma_rate = 0

    zeroes = 0
    ones = 0

    for position in range(0, line_width):
        for _, line in enumerate(rows):
            match line[position]:
                case '0':
                    zeroes += 1
                case '1':
                    ones += 1

        if position > 0:
            gamma_rate <<= 1

        if ones > zeroes:
            gamma_rate |= 1

        ones = 0
        zeroes = 0

    return gamma_rate


def solution(filename: str) -> int:
    with open(filename, 'r') as infile:
        rows = [n for n in infile.read().splitlines()]
        infile.close()

        line_width = len(rows[0])

    return get_gamma_rate(rows, line_width) * get_epsilon_rate(rows, line_width)


if __name__ == '__main__':
    try:
        print(f"What is the power consumption of the submarine? {solution(sys.argv[1])}")
    except IndexError as e:
        print("No file name to read input from was provided")
    except FileNotFoundError as e:
        print(f"{sys.argv[1]}: {e.args[1]}")
