def get_life_support_generator_rating(get_rating_for_co2: bool, r: list, line_width: int, column: int) -> list:
    list_with_ones_common = [elem for elem in r if elem[column] == '1']
    list_with_zeros_common = [elem for elem in r if elem[column] == '0']

    ret = []

    if get_rating_for_co2:
        if len(list_with_zeros_common) <= len(list_with_ones_common):
            ret = list_with_zeros_common
        else:
            ret = list_with_ones_common
    else:
        if len(list_with_ones_common) >= len(list_with_zeros_common):
            ret = list_with_ones_common
        else:
            ret = list_with_zeros_common

    if len(ret) > 1:
        return get_life_support_generator_rating(get_rating_for_co2, ret, line_width, column + 1)

    return ret


def solution(filename: str) -> int:
    with open(filename, 'r') as infile:
        rows = [n for n in infile.read().splitlines()]
        infile.close()

        line_width = len(rows[0])

    return int(get_life_support_generator_rating(True, rows, line_width, 0)[0], 2) * int(get_life_support_generator_rating(False, rows, line_width, 0)[0], 2)


if __name__ == '__main__':
    print(f"What is the life support rating of the submarine? {solution('test_input')} - using test_input")
    print(f"What is the life support rating of the submarine? {solution('input')}")
