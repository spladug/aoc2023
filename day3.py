def is_symbol(lines: list[str], i: int, j: int) -> bool:
    if i < 0 or j < 0:
        return False

    try:
        c = lines[i][j]
    except IndexError:
        return False

    # print(f"{i=} {j=} {c=} {c.isdigit()=}")
    return not c.isdigit() and c != "."


def is_symbol_abovebelow(lines: list[str], i: int, j: int) -> bool:
    return any(is_symbol(lines, i + x, j) for x in [-1, 0, 1])


def find_part_numbers(schematic: str) -> list[int]:
    lines = schematic.splitlines()

    for i in range(len(lines)):
        digits = []
        is_adjacent = False

        # print(lines[i])
        for j, c in enumerate(lines[i]):
            if c.isdigit():
                if not digits:
                    diagonal_left = is_symbol_abovebelow(lines, i, j - 1)
                    is_adjacent = diagonal_left
                    # print(f"{diagonal_left=}")

                is_adjacent |= is_symbol_abovebelow(lines, i, j)
                # print(f"digit {c=} {is_adjacent=}")
                digits.append(c)
            else:
                if digits:
                    diagonal_right = is_symbol_abovebelow(lines, i, j)
                    # print(f"not digit, {is_adjacent=} {diagonal_right=}")
                    if is_adjacent or diagonal_right:
                        # print("yield")
                        yield int("".join(digits))
                else:
                    # print("not digit, no digits")
                    pass

                digits.clear()
                is_adjacent = False

        if digits and is_adjacent:
            yield int("".join(digits))


TEST = """\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""

print(list(find_part_numbers(TEST)))
assert sum(find_part_numbers(TEST)) == 4361

with open("input") as f:
    puzzle = f.read()

part_numbers = list(find_part_numbers(puzzle))
print(part_numbers)
print(sum(part_numbers))
