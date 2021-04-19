#include "integer.h"

extern "C" {

Integer *integer_new(int value);

void integer_delete(Integer const *__self);

int integer_get(Integer const *__self);

void integer_set(Integer *__self, int value);

}
