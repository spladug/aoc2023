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


def find_full_part_number(schematic: list[str], i: int, j: int) -> int:
    assert schematic[i][j].isdigit()

    while j > 0:
        if not schematic[i][j - 1].isdigit():
            break
        j -= 1

    digits = []
    for c in schematic[i][j:]:
        if not c.isdigit():
            break
        digits.append(c)
    return int("".join(digits))


assert find_full_part_number(["...a..123..."], 0, 7) == 123


def find_adjacent_part_numbers(schematic: list[str], i: int, j: int) -> list[int]:
    part_numbers = set()
    for di in [-1, 0, 1]:
        for dj in [-1, 0, 1]:
            if not (0 <= i + di < len(schematic)):
                print(f"bailing {i+di}")
                continue
            if not (0 <= j + dj < len(schematic[0])):
                print(f"bailing {j+dj}")
                continue

            print(f"checking ({i+di},{j+dj})")
            if schematic[i + di][j + dj].isdigit():
                print(f"found potential adjacent part no: ({i+di},{j+dj})")
                part_number = find_full_part_number(schematic, i + di, j + dj)
                part_numbers.add(part_number)  # assumption, no adjacent dupes
    return list(part_numbers)


def find_gear_ratios(schematic: str) -> None:
    lines = schematic.splitlines()
    gear_ratios = []
    for i, line in enumerate(lines):
        for j, c in enumerate(line):
            if c == "*":
                part_numbers = find_adjacent_part_numbers(lines, i, j)
                print(f"found potential gear at ({i},{j}), {part_numbers=}")
                if len(part_numbers) == 2:
                    gear_ratio = part_numbers[0] * part_numbers[1]
                    print(
                        f"found gear at ({i},{j})! parts {part_numbers} => {gear_ratio=}"
                    )
                    gear_ratios.append(gear_ratio)
    return gear_ratios


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

assert sum(find_part_numbers(TEST)) == 4361
assert sum(find_gear_ratios(TEST)) == 467835

with open("input") as f:
    puzzle = f.read()

# part_numbers = list(find_part_numbers(puzzle))
# print(part_numbers)
# print(sum(part_numbers))

gear_ratios = list(find_gear_ratios(puzzle))
print(gear_ratios)
print(sum(gear_ratios))
