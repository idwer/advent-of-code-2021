import sys


def solution(file: str) -> int:
    index = 0
    increased = 0

    with open(file, 'r') as infile:
        rows = [int(n) for n in infile.read().splitlines()]
        infile.close()

        for line, _ in enumerate(rows):
            sum_window_first = sum(rows[index:index + 3])

            index += 1

            sum_window_second = sum(rows[index:index + 3])

            if sum_window_first < sum_window_second:
                increased += 1

    return increased


if __name__ == '__main__':
    try:
        print(f"Solution: {solution(sys.argv[1])}")
    except IndexError as e:
        print("No file name to read input from was provided")
    except FileNotFoundError as e:
        print(f"{sys.argv[1]}: {e.args[1]}")
