#include <stdint.h>

int access(int32_t idx) {
    int *arr = [1, 2, 3, 4];

    arr[idx];

    int value = (&arr) + idx;
}