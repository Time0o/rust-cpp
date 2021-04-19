#include "integer_rust.h"

extern "C" {

Integer *integer_new(int value)
{ return new Integer(value); }

void integer_delete(Integer const *__self)
{ delete __self; }

int integer_get(Integer const *__self)
{ return __self->get(); }

void integer_set(Integer *__self, int value)
{ __self->set(value); }

}
