#pragma once

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct vec Vec;

Vec vec_new();
void vec_delete(Vec *self);
size_t vec_len(const Vec *self);
const int *vec_last(const Vec *self);
void vec_push(Vec *self, int new_item);
int vec_pop(Vec *self);

