#ifndef clox_value_h
#define clox_value_h

#include "common.h"

typedef double Value;

// constant(s) pool - instruction to load a const looks up val by idx in our array of values
typedef struct {
    int capacity;
    int count;
    Value* values;
} ValueArray;

void initValueArray(ValueArray* array);
void writeValueArray(ValueArray* array, Value value);
void freeValueArray(ValueArray* array);
void printValue(Value value);

#endif
