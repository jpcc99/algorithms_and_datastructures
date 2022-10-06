#include "list.h"

struct list {
  int *arr;
  size_t len;
};

List *list_new(size_t capacity) {
  List *list = malloc(sizeof(List));
  list->arr = calloc(capacity, sizeof(int));
  list->len = 0;
  return list;
}

List *list_from(int *arr, size_t arr_len) {
  List *list = list_new(arr_len);
  for (size_t i = 0; i < arr_len; i++) {
    list_append(list, arr[i]);
  }
  return list;
}

void list_delete(List *self) {
  free(self->arr);
  free(self);
}

size_t list_len(const List *self) { return self->len; }

int list_get(const List *self, size_t at) { return self->arr[at]; }

void list_append(List *self, int new_item) {
  self->arr[self->len] = new_item;
  self->len += 1;
}

void move_elem_to_left(List *self, size_t at) {
  for (int i = at; i < (self->len - 1); i++) {
    self->arr[i] = self->arr[i + 1];
  }
}

int list_pop(List *self, size_t at) {
  int smallest = self->arr[at];
  if (self->len > 1) {
    move_elem_to_left(self, at);
  }
  self->len -= 1;
  return smallest;
}

void list_print(const List *self) {
  printf("[ ");
  size_t len = list_len(self);
  size_t i = 0;
  for (i = 0; len > 1; i++) {
    printf("%d, ", self->arr[i]);
    len--;
  }
  printf("%d ]\n", self->arr[i]);
}
