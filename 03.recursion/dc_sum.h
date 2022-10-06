#pragma once

#include <stdio.h>

typedef struct slice {
  int *p_arr;
  size_t len;
} Slice;

void slice_new(Slice *self, int *p_arr, size_t arr_len);

int sum(Slice slice);
int dc_sum(Slice slice);
