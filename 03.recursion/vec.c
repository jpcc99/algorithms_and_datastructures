#include "vec.h"

struct vec {
  int *p_arr;
  size_t len;
  size_t capacity;
};

Vec vec_new() {
  Vec new_vec;
  new_vec.p_arr = NULL;
  new_vec.len = 0;
  new_vec.capacity = 5;
  return new_vec;
}

void vec_delete(Vec *self) {
  free(self->p_arr);
  free(self);
}

size_t vec_len(const Vec *self) { return self->len; }

const int *vec_last(const Vec *self) {
  if (self->p_arr != NULL)
    return &self->p_arr[self->len - 1];
  else
    return NULL;
}

void push(Vec *self, int new_item) {
  self->p_arr[self->len] = new_item;
  self->len++;
}

void vec_push(Vec *self, int new_item) {
  if (self->p_arr == NULL) {
    self->p_arr = (int *)malloc(sizeof(int) * self->capacity);
  } else if (self->capacity == self->len) {
    self->capacity *= 2;
    int *new_arr = realloc(self->p_arr, sizeof(int) * self->capacity);
    if (new_arr == NULL) {
      fprintf(stderr, "[Err]: Could not reallocate array\n");
    }
    self->p_arr = new_arr;
  }
  push(self, new_item);
}

int vec_pop(Vec *self) {
  if (self->len == 0) {
    fprintf(stderr, "[Err]:Empty vec");
    return 0;
  } else {
    self->len--;
    return self->p_arr[self->len];
  }
}
