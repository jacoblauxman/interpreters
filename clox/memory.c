#include <stdlib.h>

#include "memory.h"

void* reallocate(void* pointer, size_t oldSize, size_t newSize) {
    if (newSize == 0) {
        free(pointer);
        return NULL;
    }

    void* result = realloc(pointer, newSize);
    // note: if new size is < existing block of mem, updates size and returns pointer
    // if larger, attempt to grow -> only if mem after block isnt in use, else ->
    // allocates new block of desired size, copies bytes, frees old block, returns new block ptr
    if (result == NULL) exit(1); // realloc is fallable, returns NULL if OOM
    return result;
}
