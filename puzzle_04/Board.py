class Board:
    tuples = []

    def get_sum_of_unmarked_numbers(self) -> int:
        ret = 0

        for elem in self.tuples:
            if not elem[1]:
                ret += elem[0]

        return ret

    def mark_number(self, number: int) -> None:
        for elem in self.tuples:
            if elem[0] is number:
                elem[1] = True

    def get_marked_numbers(self) -> list:
        return [elem for elem in self.tuples if elem[1]]

    def has_winning_column(self) -> bool:
        score = 0

        for n in range(0, 5 ** 2, 5):
            if self.tuples[n + 0][1] and \
                    self.tuples[n + 1][1] and \
                    self.tuples[n + 2][1] and \
                    self.tuples[n + 3][1] and \
                    self.tuples[n + 4][1]:
                return True

        return False

    def has_winning_row(self) -> bool:
        for n in range(0, 5):
            if self.tuples[n * 5 + 0][1] and \
                    self.tuples[n * 5 + 1][1] and \
                    self.tuples[n * 5 + 2][1] and \
                    self.tuples[n * 5 + 3][1] and \
                    self.tuples[n * 5 + 4][1]:
                return True

        return False

    def parse_list(self, rows: list) -> list:
        items = []

        for row in rows:
            for value in row.split():
                elem = int(value)
                items.append([elem, False])
        return items

    def __init__(self, numbers: list) -> None:
        self.tuples = self.parse_list(numbers)
