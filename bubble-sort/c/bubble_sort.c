#include <stdio.h>
#include <stdbool.h>

void bubble_sort(int numbers[10]);
bool is_sorted(int numbers[10]);
void swap(int numbers[2], int fidx, int sidx);

int main () {
    int i;
    int nums[10] = { 23, 17, 48, 12, 94, 1, 7, 13, 21, 19 };

    bubble_sort(nums);

    for (int i = 0;  i < 10; i++) {
        printf("%d\n", nums[i]);
    }

    return 0;
}

void bubble_sort(int numbers[10]) {
    int i;

    while (is_sorted(numbers) == false) {
        for (i = 0; i < 9; i++) {
            int currentn = numbers[i];
            int nextn = numbers[i + 1];

            if (currentn > nextn) {
                swap(numbers, i, i + 1);
            }
        }
    }
}

bool is_sorted(int numbers[10]) {
    bool sorted = true;
    int i;

    for (i = 0; i < 9; i++) {
        int currentn = numbers[i];
        int nextn = numbers[i + 1];

        if (currentn > nextn) {
            sorted = false;
            break;
        }
    }

    return sorted;
}

void swap(int numbers[10], int fidx, int sidx) {
    int holder;

    holder = numbers[fidx];

    numbers[fidx] = numbers[sidx];
    numbers[sidx] = holder;
}
