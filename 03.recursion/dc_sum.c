#include "dc_sum.h"

void slice_new(Slice *self, int *p_arr, size_t arr_len) {
  self->p_arr = p_arr;
  self->len = arr_len;
}

int sum(Slice slice) {
  int total = 0;
  for (size_t i = 0; i < slice.len; i++) {
    total += slice.p_arr[i];
  }
  return total;
}

int dc_sum(Slice slice) {
  if (slice.len == 0)
    return 0;
  else if (slice.len == 1)
    return slice.p_arr[0];
  Slice new_slice;
  slice_new(&new_slice, &slice.p_arr[1], slice.len - 1);
  return slice.p_arr[0] + dc_sum(new_slice);
}
