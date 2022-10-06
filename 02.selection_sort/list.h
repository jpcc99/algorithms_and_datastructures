#pragma once

#include <stdio.h>
#include <stdlib.h>

typedef struct list List;

List *list_new(size_t capacity);
List *list_from(int *arr, size_t arr_len);
int list_get(const List *self, size_t at);
void list_delete(List *self);
size_t list_len(const List *self);
size_t list_capacity(const List *self);
void list_append(List *self, int new_item);
int list_pop(List *self, size_t at);
void list_print(const List *self);
