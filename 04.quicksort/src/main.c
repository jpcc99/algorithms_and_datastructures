#include "quicksort.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define CAPACITY 1000

bool check_if_sorted(const int *arr, size_t len);

int main() {
  time_t seed = time(NULL);
  srand((unsigned int)seed);

  int random_num_arr[CAPACITY];

  for (size_t i = 0; i < CAPACITY; i++) {
    random_num_arr[i] = rand();
  }

  slice_print(random_num_arr, 0, CAPACITY);
  bool sort_check = check_if_sorted(random_num_arr, CAPACITY);
  printf("===\tThe array is sorted? %s\t===\n", sort_check ? "True" : "False");

  do_quicksort(random_num_arr, 0, CAPACITY);

  slice_print(random_num_arr, 0, CAPACITY);
  sort_check = check_if_sorted(random_num_arr, CAPACITY);
  printf("===\tThe array is sorted? %s\t===\n", sort_check ? "True" : "False");
}

bool check_if_sorted(const int *arr, size_t len) {
  for (size_t i = 1; i < len; i++) {
    if (arr[i] < arr[i - 1])
      return false;
  }
  return true;
}
