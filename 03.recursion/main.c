#include <stdio.h>

#include "dc_sum.h"
#include "vec.h"

int main() {
  // int data[] = {2, 4, 6};
  int data[] = {2, 4, 6, 12, 36, 100};
  Slice slice;
  slice_new(&slice, data, sizeof(data) / sizeof(int));
  // printf("Sum = %d\n", sum(slice)); // 12
  printf("Sum = %d\n", dc_sum(slice)); // 12
}
