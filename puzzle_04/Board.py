class Board:
    tuples = []
    score = 0
    winning_number_trigger = 0
    input_board_height = 0

    def get_sum_of_unmarked_numbers(self) -> int:
        ret = 0

        for t in self.tuples:
            if not t[1]:
                ret += t[0]

        return ret

    def mark_number(self, number: int) -> None:
    # def mark_number(self, number: int, input_board_height: int) -> None:
    #     self.input_board_height = input_board_height
    # def mark_number(self, number: int) -> None:
        for e in self.tuples:
            if e[0] is number:
                e[1] = True

                # if self.has_winning_column(input_board_height) or self.has_winning_row(input_board_height):
                #     return

    def get_marked_numbers(self) -> list:
        return [t for t in self.tuples if t[1]]

    def has_winning_column(self, input_board_height: int) -> bool:
        score = 0

        for i in range(0, input_board_height):
            # print(f"column: {i + 1}")
            for n in range(0, input_board_height ** 2, input_board_height):
                # print(self.tuples[i + n])
                if self.tuples[i + n][1]:
                    score += 1
            if score is input_board_height:
                return True

        return False

    def has_winning_row(self, input_board_height: int) -> bool:
        score = 0

        for n in range(0, input_board_height ** 2, input_board_height):
            # for i in range(0, len(self.tuples[0])):
            for i in range(0, input_board_height):
                # print(self.tuples[i + n])
                if self.tuples[n + i][1]:
                    score += 1
            if score is input_board_height:
                return True
            # print(self.tuples[n + i])

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
    #     self.input_board_height = input_board_height
        print(f"constructor in class Board, numbers = {numbers}")
        # print(numbers[0].split())
        self.tuples = self.parse_list(numbers)
