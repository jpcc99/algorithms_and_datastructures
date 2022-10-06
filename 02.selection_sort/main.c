#include "selection_sort.h"
#include <stdio.h>

int main() {
  int data[] = {5, 3, 6, 2, 10};
  size_t data_len = sizeof(data) / sizeof(int);
  List *list = list_from(data, data_len);
  list_print(list);
  list = do_selection_sort(list);
  list_print(list);
  list_delete(list);
}
