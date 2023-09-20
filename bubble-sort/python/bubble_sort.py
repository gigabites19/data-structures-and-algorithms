from typing import Any


def swap_list_elements(target_list: list[Any], index_one: int, index_two: int):
    try:
        old_one = target_list[index_one]

        target_list[index_one] = target_list[index_two]
        target_list[index_two] = old_one
    except IndexError:
        print('List index out of range.')


def bubble_sort(unsorted_numbers: list[int]) -> list[int]:
    while not is_sorted(unsorted_numbers):
        for index in range(len(unsorted_numbers) - 1):
            current_number = unsorted_numbers[index]
            next_number = unsorted_numbers[index + 1]

            if current_number > next_number:
                swap_list_elements(unsorted_numbers, index, index + 1)

    return unsorted_numbers


def is_sorted(maybe_sorted_numbers: list[int]) -> bool:
    _sorted = True

    for index in range(len(maybe_sorted_numbers) - 1):
        current_number = maybe_sorted_numbers[index]
        next_number = maybe_sorted_numbers[index + 1]

        if current_number > next_number:
            _sorted = False
            break

    return _sorted


unsorted_list = [1, 19, 22, 1, 2, 5, 3, 99, 127, 12, 4]

sorted_list = bubble_sort(unsorted_list)

print(sorted_list)

