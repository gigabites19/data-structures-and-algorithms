#include <math.h>
#include <stdio.h>

int binary_search(int needle, int haystack[15], int start_idx, int end_idx);
int get_mid(int one, int two);

int main() {
    int numbers[15] = { -23, -12, -10, 0, 5, 23, 48, 75, 99, 102, 193, 201, 233, 234, 300 };

    int array_length = sizeof(numbers) / sizeof(numbers[0]);

    int needle = 235;

    printf("Looking for %d\n\n", needle);
    int needle_idx = binary_search(needle, numbers, 0, array_length - 1);

    if (needle_idx == -1) {
        printf("\nCould not find %d in the array\n", needle);
    } else {
        printf("\nIndex of %d is %d\n", needle, needle_idx);
    }

    return 0;
}

int binary_search(int needle, int haystack[15], int start_idx, int end_idx) {
    int possibly_needle;
    int mid = get_mid(start_idx, end_idx);

    printf("start:%d\tend:%d\tmid:%d\n", start_idx, end_idx, mid);

    possibly_needle = haystack[mid];

    if (end_idx - start_idx == 0 && possibly_needle != needle) {
        return -1;
    } 

    if (possibly_needle == needle) {
        return mid;
    } else if (possibly_needle < needle) {
        start_idx = mid + 1;
        return binary_search(needle, haystack, start_idx, end_idx);
    } else if (possibly_needle > needle) {
        end_idx = mid;
        return binary_search(needle, haystack, start_idx, end_idx);
    } else {
        return -1;
    }

    return 1;
}

int get_mid(int one, int two) {
    // no need to floor because division by int truncates
    return (one + two) / 2;
}

