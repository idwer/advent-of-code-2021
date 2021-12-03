import sys


def solution(file):
    increased = 0

    with open(file) as infile:
        rows = [int(n) for n in infile.read().splitlines()]

        for line, _ in enumerate(rows):
            if rows[line] > rows[line - 1]:
                increased += 1

    print(f"solution: {increased}")


if __name__ == '__main__':
    solution(sys.argv[1])
