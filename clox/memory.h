#ifndef clox_memory_h
#define clox_memory_h

#include "common.h"

// macro to calc new cap based on given curr cap
#define GROW_CAPACITY(capacity) \
    ((capacity) < 8 ? 8 : (capacity) * 2)

// macro to help 'reallocate' call - gets size of array's ele type and casts resulting `void*` back to correct pointer type
#define GROW_ARRAY(type, pointer, oldCount, newCount) \
    (type*)reallocate(pointer, sizeof(type) * (oldCount), \
        sizeof(type) * (newCount))

// anotherwrapper over reallocate, just frees mem used in dyn. arr.
#define FREE_ARRAY(type, pointer, oldCount) \
    reallocate(pointer, sizeof(type) * (oldCount), 0)


    // used for all dynamic memory management re: clox
void* reallocate(void* pointer, size_t oldSize, size_t newSize);
// note: routing allocating/freeing/adjusting size of mem management helps re: GC later

#endif
