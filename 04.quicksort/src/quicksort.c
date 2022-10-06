#include "quicksort.h"

void do_quicksort(int *arr, size_t low, size_t high);
size_t get_partition_point(int *arr, size_t start, size_t end);
void swap(int *a, int *b);

void slice_print(const int *arr, size_t start, size_t len) {
  printf("\t[\n");
  for (size_t i = 0; i < len; i++) {
    printf("%d, ", arr[i]);
  }
  printf("%d\n\t]\n", arr[len - 1]);
}

void do_quicksort(int *arr, size_t low, size_t high) {
  if (low >= high)
    return;
  size_t pp = get_partition_point(arr, low, high);
  do_quicksort(arr, low, pp);
  do_quicksort(arr, pp + 1, high);
}

size_t get_partition_point(int *arr, size_t start, size_t end) {
  size_t low = start;
  size_t high = end - 1;
  int pivot = arr[high];

  for (size_t i = start; i < high; i++) {
    if (arr[i] <= pivot) {
      swap(&arr[low], &arr[i]);
      low++;
    }
  }

  swap(&arr[high], &arr[low]);
  return low;
}

void swap(int *a, int *b) {
  int temp = *a;
  *a = *b;
  *b = temp;
}
