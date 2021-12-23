class Board:
    tuples = []
    score = 0
    winning_number_trigger = 0

    def get_sum_of_unmarked_numbers(self) -> int:
        ret = 0

        for t in self.tuples:
            if not t[1]:
                ret += t[0]

        return ret

    def mark_number(self, number: int) -> None:
        for e in self.tuples:
            if e[0] is number:
                e[1] = True

    def get_marked_numbers(self) -> list:
        return [t for t in self.tuples if t[1]]

    def has_winning_column(self) -> bool:
        score = 0

        for i in range(0, 5):
            for n in range(0, 5 ** 2, 5):
                # print(self.tuples[i + n])
                if self.tuples[i + n][1]:
                    score += 1
            if score == 5:
                return True

        return False

    def has_winning_row(self) -> bool:
        score = 0

        for n in range(0, 5):
            if self.tuples[n * 5 + 0][1] and \
                self.tuples[n * 5 + 1][1] and \
                self.tuples[n * 5 + 2][1] and \
                self.tuples[n * 5 + 3][1] and \
                self.tuples[n * 5 + 4][1]:
                    print(self.tuples[n:n + 5])
                    return True

        return False

    def get_score(self) -> int:
        return self.get_sum_of_unmarked_numbers() * self.winning_number_trigger

    def parse_list(self, rows: list) -> list:
        t = []

        for row in rows:
            for value in row.split():
                elem = int(value)
                t.append([elem, False])
        return t

    def __init__(self, numbers: list) -> None:
    # def __init__(self, numbers: list, input_board_height: int) -> None:
        print(f"constructor in class Board, numbers = {numbers}")
        # print(numbers[0].split())
        self.tuples = self.parse_list(numbers)
