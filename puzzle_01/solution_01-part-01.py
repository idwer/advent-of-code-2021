def solution(file: str) -> int:
    increased = 0

    with open(file, 'r') as infile:
        rows = [int(n) for n in infile.read().splitlines()]
        infile.close()

        for line, _ in enumerate(rows):
            if rows[line] > rows[line - 1]:
                increased += 1

    return increased


if __name__ == '__main__':
    print(f'Solution: {solution("input" )}')
    print(f'Solution: {solution("test_input" )} - using test_input')