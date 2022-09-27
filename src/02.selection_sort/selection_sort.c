#include "selection_sort.h"

size_t find_smallest_index_wth(List *list) {
  int smallest = list_get(list, 0);
  size_t smallest_idx = 0;
  for (size_t i = 0; i < list_len(list); i++) {
    int curret_item = list_get(list, i);
    if (curret_item < smallest) {
      smallest_idx = i;
      smallest = curret_item;
    }
  }
  return smallest_idx;
}

List *do_selection_sort(List *list) {
  List *new_list = list_new(list_len(list));
  while (list_len(list) > 0) {
    size_t smallest_idx = find_smallest_index_wth(list);
    list_append(new_list, list_pop(list, smallest_idx));
  }
  free(list);
  return new_list;
}
