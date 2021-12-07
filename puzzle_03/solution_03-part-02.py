def get_co2_scrubber_rating(r: list, line_width: int, column: int) -> list:
    list_with_ones_common = [elem for elem in r if elem[column] == '1']
    list_with_zeros_common = [elem for elem in r if elem[column] == '0']

    ret = []

    if len(list_with_ones_common) < len(list_with_zeros_common):
        ret = list_with_ones_common
    elif len(list_with_ones_common) == len(list_with_zeros_common):
        ret = list_with_zeros_common
    else:
        ret = list_with_zeros_common

    if len(ret) > 1:
        return get_co2_scrubber_rating(ret, line_width, column + 1)

    return ret


def get_oxygen_generator_rating(r: list, line_width: int, column: int) -> list:
    list_with_ones_common = [elem for elem in r if elem[column] == '1']
    list_with_zeros_common = [elem for elem in r if elem[column] == '0']

    ret = []

    if len(list_with_ones_common) > len(list_with_zeros_common):
        ret = list_with_ones_common
    elif len(list_with_ones_common) == len(list_with_zeros_common):
        ret = list_with_ones_common
    else:
        ret = list_with_zeros_common

    if len(ret) > 1:
        return get_oxygen_generator_rating(ret, line_width, column + 1)

    return ret


def solution() -> int:
    with open('input', 'r') as infile:
        rows = [n for n in infile.read().splitlines()]
        infile.close()

        line_width = len(rows[0])

    return int(get_co2_scrubber_rating(rows, line_width, 0)[0], 2) * int(get_oxygen_generator_rating(rows, line_width, 0)[0], 2)


if __name__ == '__main__':
    print(f"What is the life support rating of the submarine? {solution()}")
