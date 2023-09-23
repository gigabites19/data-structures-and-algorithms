import math

def binary_search(needle: int, haystack: list[int], start_idx: int, end_idx: int) -> int:
    mid = get_mid(start_idx, end_idx)

    maybe_needle = haystack[mid]

    if (end_idx - start_idx) == 0 and maybe_needle != needle:
        return -1

    if maybe_needle == needle:
        return mid
    elif maybe_needle < needle:
        start_idx = mid + 1
        return binary_search(needle, haystack, start_idx, end_idx)
    elif maybe_needle > needle:
        end_idx = mid
        return binary_search(needle, haystack, start_idx, end_idx)
    else:
        return -1


def get_mid(n_one: int, n_two: int) -> int:
    return int(math.floor((n_one + n_two) / 2))

haystack = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 18]
needle = 18 

needle_idx = binary_search(needle, haystack, 0, len(haystack) - 1)

print(needle_idx)

